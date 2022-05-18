# Membership admin with gPRC

This is a demo which demonstrates using gPRC with microservices.

## Design

| Event         | Client | Server  | Response                                      |
|:--------------|:------:|:-------:|:----------------------------------------------|
| AddMember     | Member | Worker  | <- AddMemberResponse <br> -> PrintCard        |
| LicenseMember | Member | Worker  | <- LicenseMemberResponse <br> -> PrintLicense |
| PrintCard     | Worker |  Card   | <- PrintResponse                              |
| PrintLicense  | Worker | License | <- PrintResponse                              |

>Table Abbreviations
>
> * `<-` is response/feedback of an event
> * `->` is calling another event

<br>

**Services Port guide**
| Service         | Port  |
|:----------------|:-----:|
| server (worker) | 50051 |
| card            | 50052 |
| license         | 50053 |

**API IP**
| API       |           IP            | Link                                 |
|:----------|:-----------------------:|:-------------------------------------|
| /register | <http://localhost:8000> | <http://localhost:8000/api/register> |


---

## Run project

```bash
# Working Directory
$ pwd
.../grpc_membersip

# Run server (worker)
$ cd server/
cargo run

# Run member service
$ cd member/
cargo run

# Run card service
$ cd card/
cargo run

# Run license service
$ cd license/
cargo run


```




Sample API:

```http
POST http://localhost:8000/api/register HTTP/1.1
content-type: application/json

{
    "name": "Ahmed Ali",
    "email": "ahmed.ali@email.co",
    "username": "ahmed.ali"
}
```