#!/usr/bin/env sh

# Setup
workdir=$(git rev-parse --show-toplevel)
cert_dir=$workdir/data/certs
scripts_dir=$workdir/scripts
mkdir -p "$cert_dir"
rm "$cert_dir"/*

# Generate CA key and cert
openssl genpkey -algorithm RSA -out "$cert_dir"/cakey.pem
openssl req -x509 -new -key "$cert_dir"/cakey.pem -sha256 -days 3650 -out "$cert_dir"/cacert.pem -subj /CN=sixdegreesofpgp-ca

# Generate server key and CSR with SANs
openssl genpkey -algorithm RSA -out "$cert_dir"/serverkey.pem
openssl req -new -key "$cert_dir"/serverkey.pem -out "$cert_dir"/server.csr -config "$scripts_dir"/server_cert.conf

# Sign the server certificate with the CA and SANs
openssl x509 -req -in "$cert_dir"/server.csr -CA "$cert_dir"/cacert.pem -CAkey "$cert_dir"/cakey.pem -CAcreateserial -out "$cert_dir"/servercert.pem -days 365 -sha256 -extfile "$scripts_dir"/server_cert.conf -extensions req_ext
cp "$cert_dir"/servercert.pem "$cert_dir"/fullchain.pem

# Cleanup
rm "$cert_dir"/cacert.srl "$cert_dir"/server.csr

# Finalize
chmod 777 "$cert_dir"/*
ca_cert_content=$(awk 'NF {sub(/\r/, ""); printf "        %s\\n", $0;}' "$cert_dir"/cacert.pem)
sed -i "s|# - LDAP_TLS_CACERT.*|- LDAP_TLS_CACERT=>\n$ca_cert_content|" "$workdir/compose.yml"
