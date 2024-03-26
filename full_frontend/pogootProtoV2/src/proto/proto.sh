protoc -I ./ \
--js_out=import_style=commonjs:./ \
--grpc-web_out=import_style=commonjs,mode=grpcwebtext:./ \
./pogoots.proto


# --go_out=plugins=grpc,paths=source_relative:/goclient \
