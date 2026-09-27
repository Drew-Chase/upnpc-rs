# Maintainer: Drew Chase <dcmanproductions@gmail.com>
pkgname=upnpc-rs
pkgver=1.0.0
pkgrel=1
pkgdesc="A fast, lightweight, and easy to use CLI for managing your network's UPnP port mappings"
arch=('x86_64' 'aarch64')
url="https://github.com/Drew-Chase/upnpc-rs"
license=('MIT')   # TODO: add a LICENSE file to the repo and make this match it
depends=('gcc-libs' 'glibc')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')   # run `updpkgsums` to fill this in

prepare() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --locked --target "$(rustc --print host-tuple)"
}

build() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --bin upnpc
}

check() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen --workspace
}

package() {
  cd "$pkgname-$pkgver"
  # Installed as upnpc-rs: /usr/bin/upnpc is already owned by extra/miniupnpc
  install -Dm755 target/release/upnpc "$pkgdir/usr/bin/upnpc-rs"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
  # install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
