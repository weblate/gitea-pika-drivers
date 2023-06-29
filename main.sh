# Clone Upstream
mkdir -p pika-drivers
cp -rvf ./* ./pika-drivers/
cd ./pika-drivers/

# Get build deps
apt-get build-dep ./ -y

# Build package
dpkg-buildpackage --no-sign

# Move the debs to output
cd ../
mkdir -p ./output
mv ./*.deb ./output/
