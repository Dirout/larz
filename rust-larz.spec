# Generated by rust2rpm 23
%bcond_without check

%global crate larz

Name:           rust-larz
Version:        0.3.2
Release:        %autorelease
Summary:        Archive tool for efficient decompression

License:        AGPL-3.0-or-later
URL:            https://crates.io/crates/larz
Source:         %{crates_source}

BuildRequires:  rust-packaging >= 21

%global _description %{expand:
Archive tool for efficient decompression.}

%description %{_description}

%package     -n %{crate}
Summary:        %{summary}

%description -n %{crate} %{_description}

%files       -n %{crate}
%license COPYING
%license LICENSE.md
%license NOTICE
%doc CODE_OF_CONDUCT.md
%doc CONTRIBUTING.md
%doc README
%doc SECURITY.md
%{_bindir}/larz

%package        devel
Summary:        %{summary}
BuildArch:      noarch

%description    devel %{_description}

This package contains library source intended for building other packages which
use the "%{crate}" crate.

%files          devel
%license %{crate_instdir}/COPYING
%license %{crate_instdir}/LICENSE.md
%license %{crate_instdir}/NOTICE
%doc %{crate_instdir}/CODE_OF_CONDUCT.md
%doc %{crate_instdir}/CONTRIBUTING.md
%doc %{crate_instdir}/README
%doc %{crate_instdir}/SECURITY.md
%{crate_instdir}/

%package     -n %{name}+default-devel
Summary:        %{summary}
BuildArch:      noarch

%description -n %{name}+default-devel %{_description}

This package contains library source intended for building other packages which
use the "default" feature of the "%{crate}" crate.

%files       -n %{name}+default-devel
%ghost %{crate_instdir}/Cargo.toml

%package     -n %{name}+bin-devel
Summary:        %{summary}
BuildArch:      noarch

%description -n %{name}+bin-devel %{_description}

This package contains library source intended for building other packages which
use the "bin" feature of the "%{crate}" crate.

%files       -n %{name}+bin-devel
%ghost %{crate_instdir}/Cargo.toml

%package     -n %{name}+clap-devel
Summary:        %{summary}
BuildArch:      noarch

%description -n %{name}+clap-devel %{_description}

This package contains library source intended for building other packages which
use the "clap" feature of the "%{crate}" crate.

%files       -n %{name}+clap-devel
%ghost %{crate_instdir}/Cargo.toml

%package     -n %{name}+clean-path-devel
Summary:        %{summary}
BuildArch:      noarch

%description -n %{name}+clean-path-devel %{_description}

This package contains library source intended for building other packages which
use the "clean-path" feature of the "%{crate}" crate.

%files       -n %{name}+clean-path-devel
%ghost %{crate_instdir}/Cargo.toml

%package     -n %{name}+home-devel
Summary:        %{summary}
BuildArch:      noarch

%description -n %{name}+home-devel %{_description}

This package contains library source intended for building other packages which
use the "home" feature of the "%{crate}" crate.

%files       -n %{name}+home-devel
%ghost %{crate_instdir}/Cargo.toml

%package     -n %{name}+lazy_static-devel
Summary:        %{summary}
BuildArch:      noarch

%description -n %{name}+lazy_static-devel %{_description}

This package contains library source intended for building other packages which
use the "lazy_static" feature of the "%{crate}" crate.

%files       -n %{name}+lazy_static-devel
%ghost %{crate_instdir}/Cargo.toml

%package     -n %{name}+mimalloc-devel
Summary:        %{summary}
BuildArch:      noarch

%description -n %{name}+mimalloc-devel %{_description}

This package contains library source intended for building other packages which
use the "mimalloc" feature of the "%{crate}" crate.

%files       -n %{name}+mimalloc-devel
%ghost %{crate_instdir}/Cargo.toml

%package     -n %{name}+safe-devel
Summary:        %{summary}
BuildArch:      noarch

%description -n %{name}+safe-devel %{_description}

This package contains library source intended for building other packages which
use the "safe" feature of the "%{crate}" crate.

%files       -n %{name}+safe-devel
%ghost %{crate_instdir}/Cargo.toml

%package     -n %{name}+streaming-devel
Summary:        %{summary}
BuildArch:      noarch

%description -n %{name}+streaming-devel %{_description}

This package contains library source intended for building other packages which
use the "streaming" feature of the "%{crate}" crate.

%files       -n %{name}+streaming-devel
%ghost %{crate_instdir}/Cargo.toml

%package     -n %{name}+ticky-devel
Summary:        %{summary}
BuildArch:      noarch

%description -n %{name}+ticky-devel %{_description}

This package contains library source intended for building other packages which
use the "ticky" feature of the "%{crate}" crate.

%files       -n %{name}+ticky-devel
%ghost %{crate_instdir}/Cargo.toml

%package     -n %{name}+wild-devel
Summary:        %{summary}
BuildArch:      noarch

%description -n %{name}+wild-devel %{_description}

This package contains library source intended for building other packages which
use the "wild" feature of the "%{crate}" crate.

%files       -n %{name}+wild-devel
%ghost %{crate_instdir}/Cargo.toml

%prep
%autosetup -n %{crate}-%{version_no_tilde} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%changelog
%autochangelog
