# Generated by rust2rpm 23
%bcond_without check

%global crate larz

Name:           rust-larz
Version:        0.3.1
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
