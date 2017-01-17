pkg_name=conveyor
pkg_origin=chef
pkg_version=0.0.1
pkg_license=('Apache-2.0')
pkg_upstream_url=https://github.com/kmacgugan/conveyor
pkg_maintainer=kmacgugan@chef.io
pkg_shasum=d7db7aae5e7ea85a17fa24b06c68a9bfe11cd81d03765b402276bc8a62b88c4f
pkg_source=localsource.tar.gz
pkg_deps=(core/gcc core/libarchive core/openssl)
pkg_build_deps=(core/rust core/cacerts core/zlib)
pkg_bin_dirs=(bin)
program=conveyor

do_download() {
  return 0
}

do_strip() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}

do_prepare() {
  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  # Used to find libgcc_s.so.1 when compiling `build.rs` in dependencies. Since
  # this used only at build time, we will use the version found in the gcc
  # package proper--it won't find its way into the final binaries.
  export LD_LIBRARY_PATH="$(pkg_path_for gcc)/lib"
  build_line "Setting LD_LIBRARY_PATH=$LD_LIBRARY_PATH"

  la_ldflags="-L$(pkg_path_for zlib)/lib -lz"
  la_ldflags="$la_ldflags -L$(pkg_path_for openssl)/lib -lssl -lcrypto"

  export LIBARCHIVE_LIB_DIR="$(pkg_path_for libarchive)/lib"
  export LIBARCHIVE_INCLUDE_DIR="$(pkg_path_for libarchive)/include"
  export LIBARCHIVE_LDFLAGS="$la_ldflags"
  export OPENSSL_LIB_DIR="$(pkg_path_for openssl)/lib"
  export OPENSSL_INCLUDE_DIR="$(pkg_path_for openssl)/include"
}

do_build() {
  mkdir -p "${pkg_name}"
  cp -r "${PLAN_CONTEXT}/../app/." "${pkg_name}"
  cd "${pkg_name}"
  export CPPFLAGS="${CPPFLAGS} ${CFLAGS}"
  export OUT_DIR="codegen"
  cargo clean --verbose
  cargo build --release --verbose
}

do_install() {
  mkdir $pkg_prefix/bin
  cp $HAB_CACHE_SRC_PATH/$pkg_dirname/$program/target/release/$program $pkg_prefix/bin/$program
  cp -r $HAB_CACHE_SRC_PATH/$pkg_dirname/$program/data $pkg_prefix/data
}

do_strip() {
  strip $pkg_prefix/bin/$program
}
