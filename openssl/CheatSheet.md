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
openssl rsa -in private.key -pubout -out public-key.pem
```

### Generate ECDSA keys
