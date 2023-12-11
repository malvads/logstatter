Name:      rb-logstatter
Version:   0.0.1
Release:   1%{?dist}
BuildArch: noarch
Summary:   RedBorder Python AI Outliers Detection Service

License:   AGPL-3.0
URL:       https://github.com/malvads/logstatter
Source0:   %{name}-%{version}.tar.gz

%description
%{summary}

%prep
%autosetup

%build
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

cargo build --release

%install
install -D target/release/logstatter %{buildroot}/usr/bin/logstatter

%files
%doc
/usr/bin/my-rust-app

%changelog
* Date Your Name <your.email@example.com> - 1.0-1
- Initial package release

