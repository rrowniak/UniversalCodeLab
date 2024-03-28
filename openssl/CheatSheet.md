# OpenSSL Cheat Sheet

## Generate keys

### Generate RSA keys

The key size can be e.g. 2048, 4096, 8192

```
openssl genrsa -out private.key 4096
```

Generate a key that is a pass phase protected:

```
openssl genrsa -aes256 -out private.key 4096
```

Remove passphrase protection from the key:
```
openssl rsa -in private.key -out private.key
```

Print to file a corresponding public key:

```
openssl rsa -in private.key -pubout -out public.key
```

### Generate ECDSA keys

First, list available curves:

```
openssl ecparam -list_curves
```

Generate a key:

```
openssl ecparam -name prime256v1 -genkey -noout -out private.ec.key
```

Print to file a corresponding public key:

```
openssl ec -in private.ec.key -pubout -out public.ec.key
```

## Generate CA certificate
First, create a simple config file `ca_config.cnf`:

```
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
```

Create a self-signed CA certificate:

```
openssl req -config ca_config.cnf -extensions 'ca_extensions' -new -x509 -sha256 -key private.key -out ca_cert.pem -days 360
```

Check the details of the certificate:

```
openssl x509 -in ca_cert.pem -text -noout
```

## Create TLS certificate

### Create Certificate Signing Request

## Convert certificates to other formats

Convert a certificate to the crt format (supported on e.g. Windows platforms):

```
openssl x509 -outform der -in cert.pem -out cert.crt
```

Create a pfx chain:

```
openssl pkcs12 -export -inkey private.key -in ca_cert.pem -out ca_chain.pfx

