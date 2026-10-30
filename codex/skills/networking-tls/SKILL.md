---
name: networking-tls
description: "Trigger: tls, ssl, certificate, openssl, mTLS, nginx, reverse proxy, https, letsencrypt, certbot. Networking & TLS — OpenSSL, certificates, mTLS, nginx reverse proxy, Let's Encrypt, HTTPS configuration, proxy protocols."
---

# Networking & TLS

## OpenSSL

### Certificate Management
```bash
# Generate a self-signed certificate (RSA)
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
  -days 365 -nodes -subj "/CN=localhost"

# Generate a self-signed certificate (ECDSA — faster)
openssl ecparam -genkey -name prime256v1 -out key.pem
openssl req -new -key key.pem -x509 -days 365 -out cert.pem \
  -subj "/CN=localhost"

# Generate a CSR (Certificate Signing Request)
openssl req -new -newkey rsa:2048 -nodes \
  -keyout private.key -out request.csr \
  -subj "/C=US/ST=State/L=City/O=Company/CN=example.com"

# View certificate details
openssl x509 -in cert.pem -text -noout
openssl x509 -in cert.pem -subject -issuer -dates -noout

# View CSR details
openssl req -in request.csr -text -noout

# Check certificate expiry
openssl x509 -in cert.pem -noout -enddate

# Verify certificate chain
openssl verify -CAfile ca.pem cert.pem
openssl verify -CAfile ca.pem -untrusted intermediate.pem cert.pem
```

### Convert Formats
```bash
# PEM → DER
openssl x509 -in cert.pem -outform der -out cert.der

# DER → PEM
openssl x509 -in cert.der -inform der -out cert.pem

# PEM → PKCS12 (for Java keystores)
openssl pkcs12 -export -in cert.pem -inkey key.pem \
  -out keystore.p12 -name myalias -password pass:changeit

# PKCS12 → PEM
openssl pkcs12 -in keystore.p12 -nodes -out bundle.pem

# PEM → JKS (via PKCS12 intermediate)
keytool -importkeystore -srckeystore keystore.p12 -srcstoretype PKCS12 \
  -destkeystore keystore.jks -deststoretype JKS
```

### Testing & Debugging
```bash
# Test TLS connection
openssl s_client -connect example.com:443

# Show full certificate chain
openssl s_client -showcerts -connect example.com:443

# Test SNI
openssl s_client -connect example.com:443 -servername example.com

# Test specific TLS version
openssl s_client -tls1_2 -connect example.com:443
openssl s_client -tls1_3 -connect example.com:443

# Check OCSP stapling
openssl s_client -status -connect example.com:443

# Generate a random password
openssl rand -base64 32
```

## Let's Encrypt / Certbot

```bash
# Install certbot
sudo apt install certbot python3-certbot-nginx

# Get certificate (standalone mode)
sudo certbot certonly --standalone -d example.com -d www.example.com

# Get certificate (nginx mode)
sudo certbot --nginx -d example.com

# Renew all certificates
sudo certbot renew

# Renew specific certificate
sudo certbot renew --cert-name example.com

# Dry run renewal test
sudo certbot renew --dry-run

# Automatic renewal (systemd timer)
sudo systemctl status certbot.timer
# /lib/systemd/system/certbot.timer runs twice daily
# Certificates are renewed if < 30 days to expiry

# List certificates
sudo certbot certificates

# Revoke a certificate
sudo certbot revoke --cert-name example.com

# Certificate locations:
/etc/letsencrypt/live/example.com/
├── fullchain.pem     # Certificate + intermediate chain
├── privkey.pem       # Private key
├── cert.pem          # Certificate only
└── chain.pem         # Intermediate chain only
```

## mTLS (Mutual TLS)

### Generate CA, Server, and Client Certs
```bash
# 1. Create Certificate Authority
openssl genrsa -out ca-key.pem 4096
openssl req -x509 -new -nodes -key ca-key.pem -days 3650 \
  -out ca-cert.pem -subj "/CN=MyCA"

# 2. Generate server cert signed by CA
openssl genrsa -out server-key.pem 2048
openssl req -new -key server-key.pem -out server.csr \
  -subj "/CN=localhost"
openssl x509 -req -in server.csr -CA ca-cert.pem -CAkey ca-key.pem \
  -CAcreateserial -out server-cert.pem -days 365

# 3. Generate client cert signed by CA
openssl genrsa -out client-key.pem 2048
openssl req -new -key client-key.pem -out client.csr \
  -subj "/CN=client"
openssl x509 -req -in client.csr -CA ca-cert.pem -CAkey ca-key.pem \
  -CAcreateserial -out client-cert.pem -days 365
```

### Spring Boot mTLS Configuration
```yaml
# application.yml
server:
  ssl:
    enabled: true
    client-auth: need                    # require client certificate
    key-store: classpath:certs/server-keystore.p12
    key-store-password: changeit
    key-store-type: PKCS12
    trust-store: classpath:certs/truststore.p12
    trust-store-password: changeit
    trust-store-type: PKCS12
```

### Test mTLS with curl
```bash
curl --cacert ca-cert.pem \
  --cert client-cert.pem \
  --key client-key.pem \
  https://localhost:8443/api/v1/orders
```

## Nginx Reverse Proxy

```nginx
server {
    listen 80;
    server_name example.com;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl http2;
    server_name example.com;

    # TLS
    ssl_certificate     /etc/letsencrypt/live/example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/example.com/privkey.pem;

    # Modern TLS config
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256;
    ssl_prefer_server_ciphers on;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;

    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Content-Type-Options nosniff;
    add_header X-Frame-Options DENY;
    add_header X-XSS-Protection "0";

    # Proxy to backend
    location /api/ {
        proxy_pass http://backend:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Timeouts
        proxy_connect_timeout 60s;
        proxy_read_timeout 60s;
        proxy_send_timeout 60s;
    }

    # Static files
    location / {
        root /usr/share/nginx/html;
        try_files $uri $uri/ /index.html;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Deny access to hidden files
    location ~ /\. {
        deny all;
    }
}
```

### Rate Limiting (nginx)
```nginx
# Define limit zone
limit_req_zone $binary_remote_addr zone=api:10m rate=100r/s;

server {
    location /api/ {
        limit_req zone=api burst=20 nodelay;
        proxy_pass http://backend:8080;
    }
}
```

## HTTP Security Headers

```
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self'
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 0
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: geolocation=(), microphone=(), camera=()
Cache-Control: no-store
```

## Network Diagnostics

```bash
# Full network stats
ss -tulpn                       # all listening TCP/UDP
ss -tan                         # all TCP connections
ss -tup                         # all TCP/UDP with process

# Route tracing
mtr example.com                 # continuous traceroute
tracepath example.com           # MTU discovery

# Bandwidth test
iperf3 -c <server>              # client
iperf3 -s                       # server

# DNS lookup
dig example.com ANY
dig example.com A +short
dig -x 8.8.8.8                  # reverse DNS

# Check DNS propagation
whois example.com

# Packet capture
sudo tcpdump -i eth0 port 443
sudo tcpdump -i eth0 host 10.0.0.1
sudo tcpdump -i eth0 -w capture.pcap

# HTTP debugging with curl
curl -v https://example.com/api
curl -i https://example.com/api                  # include response headers
curl -w "\n%{http_code}\n" https://example.com   # just status code
curl -o /dev/null -s -w "%{time_total}\n" https://example.com  # response time
```
