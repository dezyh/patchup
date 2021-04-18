name=$(sed -n -E "s/name = \"(.+)\"/\1/p" Cargo.toml)
version=$(sed -n -E "s/version = \"(.+)\"/\1/p" Cargo.toml)
description=$(sed -n -E "s/description = \"(.+)\"/\1/p" Cargo.toml)
license=$(sed -n -E "s/license = \"(.+)\"/\1/p" Cargo.toml)
url=$(sed -n -E "s/repository = \"(.+)\"/\1/p" Cargo.toml)
sha256=$(sha256sum $name-cli-bin.tar.gz | cut -d ' ' -f1)
maintainer=$(sed -n -E "s/authors = \[\"(.+)\"\]/\1/p" Cargo.toml)

cat >PKGBUILD-BIN <<EOL
# Maintainer: $maintainer
pkgname=$name-bin
pkgver=$version
pkgrel=1
pkgdesc="$description"
url="$url"
license=("MIT")
arch=("x86_64")
makedepends=("cargo")
provides=("$name")
conflicts=("$name")
source=("$url/releases/download/$version/$name-cli-bin.tar.gz")
sha256sums=("$sha256")

package() {
    install -Dm755 "target/release/$name" "\$pkgdir/usr/bin/$name"    
}
EOL
