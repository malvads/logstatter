Name:      rb-logstatter
Version:   0.0.1
Release:   1%{?dist}
BuildArch: x86_64
Summary:   RedBorder Python AI Outliers Detection Service

License:   AGPL-3.0
URL:       https://github.com/malvads/logstatter
Source0:   %{name}-%{version}.tar.gz

BuildRequires: gcc openssl-devel

%global debug_package %{nil}

%description
%{summary}

%prep
%autosetup

%build
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

cargo build --release

%install
install -D target/release/logstatter %{buildroot}/usr/bin/logstatter

%files
%doc
/usr/bin/logstatter

%changelog
* Mon Dec 11 2023 Miguel Álvarez <malvarez@redborder.com> - 0.0.1-1
- Initial package release

