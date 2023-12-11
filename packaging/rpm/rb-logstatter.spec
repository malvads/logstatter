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
install -D -m 0644 src/systemd/rb-logstatter.service %{buildroot}/usr/lib/systemd/system/rb-logstatter.service
install -D -m 0644 src/etc/logstatter.conf %{buildroot}/etc/logstatter/logstatter.conf
%{__id_u} logsattter &>/dev/null || %{__useradd} -r -g logsattter -d %{_var}/lib/logsattter -s /sbin/nologin -c "logstatter user" logsattter 2>/dev/null
%{__id_g} logsattter &>/dev/null || %{__groupadd} -r logsattter 2>/dev/null

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
* Mon Dec 11 2023 Miguel Álvarez <malvarez@redborder.com> - 0.0.1-1
- Initial package release

