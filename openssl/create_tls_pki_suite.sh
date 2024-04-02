#!/bin/bash -e

# configuration
CA_PRIV_KEY=ca_priv.key
CA_CERT=ca_cert.pem
TLS_SER_PRIV_KEY=tls_serv_priv.key
TLS_SER_CERT=tls_serv.pem

# main entry point
function main {
    gen_rsa_priv_key "$CA_PRIV_KEY"
    gen_ca_certificate "$CA_PRIV_KEY" "$CA_CERT"
}

function clean_all {
    echo "Cleaning generated crypto stuff..."
    rm -f "$CA_PRIV_KEY"
    rm -f "$CA_CERT"
    rm -f "$TLS_SER_PRIV_KEY"
    rm -f "$TLS_SER_CERT"
}

if [ "$1" == "clean" ]; then
    clean_all
    exit 0
fi

function gen_rsa_priv_key {
    openssl genrsa -out "$1" 4096
}

function gen_ca_certificate {

    cat > ca_config_autogen.cnf << EOF
[ req ]
prompt = no
distinguished_name = mydn

[mydn]
commonName = my.private.lab
countryName = PL
stateOrProvinceName = malopolskie
organizationName = UniversalCodeLab
organizationalUnitName = development

[ ca_extensions ]
keyUsage = critical, digitalSignature, keyEncipherment, keyCertSign, cRLSign
basicConstraints = critical, CA:TRUE
extendedKeyUsage = critical, serverAuth, OCSPSigning
subjectKeyIdentifier = hash
EOF

    trap "rm -f ca_config_autogen.cnf" EXIT
    openssl req -config ca_config_autogen.cnf -extensions 'ca_extensions' -new -x509 -sha256 -key "$1" -out "$2" -days 360

}

function gen_csr_fqdn() {
    cat > csr_config_autogen.cnf << EOF
FQDN = $1
ORGNAME = MyOrg
ALTNAMES = DNS:\$FQDN

[ req ]
prompt = no
encrypt_key = no
distinguished_name = dn
req_extensions = ext

[ dn ]
C = PL
O = \$ORGNAME
CN = \$FQDN

[ ext ]
subjectAltName = \$ALTNAMES
EOF
    trap "rm -f csr_config_autogen.cnf" EXIT
}

main
