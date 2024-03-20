#!/bin/bash

echo "Install SBOM tool..."

OS=$(uname)
echo "OS=$OS"
ARCH="$(uname -m)"
echo "Arch=$ARCHs"

SBOM_DIR=sbom_generations
mkdir $SBOM_DIR

SBOM_LICENSES="mongo-odbc-driver.licenses.cdx.json"
SBOM_VULN="mongo-odbc-driver.merge.grype.cdx.json"
SBOM_FINAL="mongo-odbc-driver.full.cdx.json"

echo "SBOM with vulnerabilities: $SBOM_LICENSES";
echo "SBOM with license: $SBOM_VULN";
echo "Final SBOM with all information: $SBOM_FINAL"

# Install cargo-cyclonedx
echo "-- cargo-cyclonedx --"
cargo install cargo-cyclonedx
echo "------------------------------------"


# Install Grype
echo "-- Instal Grype --"
curl -sSfL https://raw.githubusercontent.com/anchore/grype/main/install.sh | sh -s -- -b $SBOM_DIR
echo "------------------------------------"


# Install CycloneDX CLI
if [ $OS = "Linux" ]; then
  if [ $ARCH = "x64" ]; then
    URL="https://github.com/CycloneDX/cyclonedx-cli/releases/download/v0.25.0/cyclonedx-linux-x64"
  elif [ $ARCH = "arm64" ]; then
    URL="https://github.com/CycloneDX/cyclonedx-cli/releases/download/v0.25.0/cyclonedx-linux-arm64"
  fi
elif [ $OS = "Darwin" ]; then
  if [ $ARCH = "arm64" ]; then
    URL="https://github.com/CycloneDX/cyclonedx-cli/releases/download/v0.25.0/cyclonedx-osx-arm64"
  else
    URL="https://github.com/CycloneDX/cyclonedx-cli/releases/download/v0.25.0/cyclonedx-osx-x64"
  fi
else
  # Windows
  URL="https://github.com/CycloneDX/cyclonedx-cli/releases/download/v0.25.0/cyclonedx-win-x64.exe"
fi

echo "-- Downloading CycloneDX CLI $URL --"
curl -L -O $URL -o cyclonedx-cli
chmod +x cyclonedx-cli
echo "------------------------------------"

echo "--  Generating SBOMs with the licenses information --"
cargo cyclonedx -v -f json  --manifest-path ../../Cargo.toml
echo "------------------------------------"

echo "-- Merging inforom both mongo-odbc-driver and win_setupgui because both are packaged libraries --"
# TODO add --version from tag
cyclonedx merge --input-files ../../odbc/mongo-odbc-driver.cdx.json ../../win_setupgui/win_setupgui.cdx.json --output-format json --input-format json --group mongo-odbc-driver --name mongo-odbc-driver> $SBOM_LICENSES
echo "------------------------------------"

echo "-- Generating SBOM with vulnerabilities information --"
grype sbom:$SBOM_LICENSES --fail-on low -o cyclonedx-json > $SBOM_VULN
SCAN_RESULT=$?
echo "------------------------------------"

echo "-- Merging the SBOMs with the licenses information and the SBOM with the  vulnerabilities information in $SBOM_FINAL --"

temp_output="temp_output.json"

if [ -f "$temp_output" ] ; then
    rm "$temp_output"
fi
touch $temp_output

while IFS= read -r line
do
  if [[ "$line" == *"purl"* ]]; then
   bash_purl=$(echo $line | cut -d '"' -f4)
   command=$(echo "jq '.components[] | select(.purl == \"$bash_purl\").licenses' $SBOM_LICENSES")
   licenseInfo=$(eval " $command")
    if [[ -z "${licenseInfo}" ]]; then
      echo "\"licenses\" : []," >> $temp_output
    else
      echo "\"licenses\" : $licenseInfo," >> $temp_output
    fi
  fi
  echo "$line" >> $temp_output

done < "$SBOM_VULN"

# Format the json file
jq . $temp_output > $SBOM_FINAL
echo "------------------------------------"

cat $SBOM_FINAL

# Exit with a failure if the scan found a vulnerability
SCAN_RESULT=$?
if [[ result -eq 0 ]]; then
  return 0
else
  return 1
fi
