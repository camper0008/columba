# specs

## create

create an alias/username for a public key

### req

```
===BEGIN_CREATE_REQ===
name
<name>
public
<public>
===END_CREATE_REQ===
```

### res

```
===BEGIN_CREATE_RES===
msg
<"success" | "exists" | "error">
===END_CREATE_RES===
```

## inbox

view inbox of user

### req

```
===BEGIN_INBOX_REQ===
name
<name>
===END_INBOX_REQ===
```

### res

```
===BEGIN_INBOX_RES===
msg
<"success">
inbox
<id> <date>
(...)
===END_INBOX_RES===
```

```
===BEGIN_INBOX_RES===
msg
<"not found" | "error">
===END_INBOX_RES===
```

## read

read an encrypted message from user

### req

```
===BEGIN_READ_REQ===
id
<post id>
name
<name>
===END_READ_REQ
```

### res

```
===BEGIN_READ_RES===
msg
<"success">
read
<encrypted message>
===END_READ_RES===
```

```
===BEGIN_READ_RES===
msg
<"not found" | "error">
===END_READ_RES===
```

## send

send a message to a user

### req

```
===BEGIN_SEND_REQ===
name
<name>
message
<message>
===END_SEND_REQ===
```

### res

```
===BEGIN_SEND_RES===
msg
<"success" | "not found" | "error">
===END_SEND_RES===
```

## whois

get publickey from alias

### req

```
===BEGIN_WHOIS_REQ===
name
<name>
===END_WHOIS_REQ===
```

### res

```
===BEGIN_WHOIS_RES===
msg
<"success">
public
<pem public key>
===END_WHOIS_RES===
```

```
===BEGIN_WHOIS_RES===
msg
<"not found" | "error">
===END_WHOIS_RES===
```
