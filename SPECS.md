# specs

## create

create an alias/username for a public key

username should be max 32 bytes

### req

```
===BEGIN_CREATE_REQ===
name
<name>
public
<public>
===END_CREATE_REQ===
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
<id> <date>
(...)
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
<encrypted message>
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
