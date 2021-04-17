name=$(sed -n -E "s/name = \"(.+)\"/\1/p" Cargo.toml)
version=$(sed -n -E "s/version = \"(.+)\"/\1/p" Cargo.toml)
description=$(sed -n -E "s/description = \"(.+)\"/\1/p" Cargo.toml)
license=$(sed -n -E "s/license = \"(.+)\"/\1/p" Cargo.toml)
url=$(sed -n -E "s/repository = \"(.+)\"/\1/p" Cargo.toml)

echo "::set-output name=name::$name"
echo "::set-output name=version::$version"
echo "::set-output name=description::$description"
echo "::set-output name=license::$license"
echo "::set-output name=url::$url"
