.PHONY: all $(MAKECMDGOALS)
.DEFAULT_GOAL=dev
dev:
	cargo watch -x run
test:
	cargo test -- --nocapture


# REQUESTS
/:
	curl ${addr}/
protected:
	curl ${post} ${addr}/protected/ -d '{}' -b token=AAAAAF3avy4AAAAAXdwQrgAAAAAAAAABZvLHEexqQgbaTTniiMEqSGlWfoAUPEpfSXRMwQErfrF05DCvVOAFO1JaC/9bld5K4xmfvlwT/cs5FQNVJ7ll6A==
users/login:
	curl ${post} ${addr}/users/login -i -d '{"email":"user.email@gmail.com", "password":"nopass"}'
users/check:
	curl ${post} ${addr}/users/check -d '{"email":"user.email@gmail.com"}'
users/register:
	curl ${post} ${addr}/users/register -d '{"email":"user.email@gmail.com","password":"nopass"}'
addr=$(if $(filter $(ENV),P),https://api-nmrshll.cloud.okteto.net,http://0.0.0.0:8080)
post= -X POST -H "Content-Type: application/json"
