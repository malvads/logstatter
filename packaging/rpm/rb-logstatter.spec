Name:      rb-logstatter
Version:   %{__version}
Release:   %{__release}%{?dist}
BuildArch: x86_64
Summary:   Logstatter service to monitor logstash
License:   AGPL-3.0
URL:       https://github.com/malvads/logstatter
Source0:   %{name}-%{version}.tar.gz
BuildRequires: perl gcc openssl-devel
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
install -D -m 0644 src/systemd/rb-logstatter.service %{buildroot}/usr/lib/systemd/system/rb-logstatter.service
install -D -m 0644 src/etc/logstatter.conf %{buildroot}/etc/logstatter/logstatter.conf
%pre
getent group logstatter >/dev/null || groupadd -r logstatter
getent passwd logstatter >/dev/null || useradd -r -g logstatter -d /var/lib/logstatter -s /sbin/nologin -c "RedBorder Logstatter User" logstatter
%post
systemctl daemon-reload
systemctl start rb-logstatter
%files
%defattr(0755,logstatter,logstatter)
/usr/bin/logstatter
%defattr(644,root,root)
/usr/lib/systemd/system/rb-logstatter.service
/etc/logstatter/logstatter.conf
%doc
%changelog
* Thu Dec 14 2023 Miguel Negron, Malvads <manegron@redborder.com, malvarez@redborder.com> - 0.0.2
* Mon Dec 11 2023 Miguel Álvarez <malvarez@redborder.com> - 0.0.1-1
- Initial package release
