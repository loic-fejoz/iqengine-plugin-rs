OPENAPI_GEN:=java -jar ~/downloads/openapi-generator-cli.jar

plugins_openapi.yaml:
	wget https://github.com/IQEngine/IQEngine/raw/main/plugins/plugins_openapi.yaml

compare:
	diff plugins_openapi.yaml plugins_openapi_adapted.yaml

generate:
	$(OPENAPI_GEN) generate -i plugins_openapi_adapted.yaml -g rust -o ./
	cargo fmt

.PHONY: generate compare plugins_openapi.yaml
