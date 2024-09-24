Name:           rhostname
Version:        0.1.0
Release:        1%{?dist}
Summary:        A tool used to perform a series of operations on usernames. It is a reconstruction of the hostname command using Rust.

License:        MulanPSL2
URL:            https://gitee.com/openeuler/rhostname
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo

%description
The `rhostname` project reimplements the classic `hostname` tool in Rust. It is used to display the system's DNS name, and to display or set its hostname or NIS domain name.

%prep
%autosetup

%build
cargo build --release

%install
install -D -m 0755 target/release/rhostname %{buildroot}/%{_bindir}/rhostname

%files
%{_bindir}/rhostname
