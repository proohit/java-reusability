package main

import (
	"log"
	"runtime"
	"unsafe"

	"tekao.net/jnigi"
)

/*
#include<server.h>
*/
import "C"

type RouteHandler struct {
	Path    string
	Handler func(string) string
}

type RouteHandlerMap map[int32]RouteHandler

var routes RouteHandlerMap

//export handle_request_external
func handle_request_external(env unsafe.Pointer, obj uintptr, raw_fn_id uintptr, raw_request_ptr uintptr) uintptr {
	req_ptr := unsafe.Pointer(raw_request_ptr)
	request := *(*string)(req_ptr)

	fn_id := int32(raw_fn_id)
	route := routes[fn_id]
	response := route.Handler(request)
	println(response)

	var env_val *jnigi.Env = jnigi.WrapEnv(env)
	resp, _ := env_val.NewObject("java/lang/String", []byte(response))

	return uintptr(unsafe.Pointer(resp))
}

func main() {
	routes = RouteHandlerMap{}
	if err := jnigi.LoadJVMLib(jnigi.AttemptToFindJVMLibPath()); err != nil {
		log.Fatal(err)
	}
	runtime.LockOSThread()
	_, env, err := jnigi.CreateJVM(jnigi.NewJVMInitArgs(false, true, int(jnigi.DEFAULT_VERSION), []string{"-Xcheck:jni", "-Djava.class.path=/home/master/workspace/java-reusability/shared-server/lib/build/classes/java/main"}))
	if err != nil {
		log.Fatal(err)
	}
	env.RegisterNative("shared/server/Server", "handle_request_external", jnigi.ObjectType("java/lang/String"), []interface{}{jnigi.Int, "java/lang/String"}, C.handle_request_external)
	handler_path1, err := env.NewObject("java/lang/String", []byte("/test1"))
	if err != nil {
		log.Fatal(err)
	}
	var handler_index1 int32
	env.CallStaticMethod("shared/server/Server", "addRoute", handler_index1, handler_path1)
	routes[handler_index1] = RouteHandler{Path: "/test1", Handler: func(request string) string {
		return "test"
	}}
	env.CallStaticMethod("shared/server/Server", "start", nil, true)

	for {
	}

}
