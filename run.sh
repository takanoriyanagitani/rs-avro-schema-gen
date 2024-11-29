#!/bin/sh

ENV_NAME=sample_schema

cat ./sample.d/sample-row.json |
	wazero \
		run \
		-env ENV_NAME="${ENV_NAME}" \
		./rs-avro-schema-gen.wasm
