extends Node
class_name WebSocketClient

@export var base_url: String = "ws://localhost:8000/ws"
@export var handshake_headers : PackedStringArray
@export var supported_protocols : PackedStringArray
@export var tls_trusted_certificate : X509Certificate
@export var tls_verify := true


var socket = WebSocketPeer.new()
var last_state = WebSocketPeer.STATE_CLOSED


signal connected_to_server()
signal connection_closed()
signal message_received(message: Variant)


func connect_to_url(url) -> int:
	socket.supported_protocols = supported_protocols
	socket.handshake_headers = handshake_headers
	var tls_options = TLSOptions.client(tls_trusted_certificate)
	var err = socket.connect_to_url(url, tls_options)
	if err != OK:
		print("i tried :(", err)
		return err
	last_state = socket.get_ready_state()
	return OK


func send(message) -> int:
	print(socket.get_ready_state())
	if typeof(message) == TYPE_STRING:
		return socket.send_text(message)
	return socket.send(var_to_bytes(message))


func get_message() -> Variant:
	if socket.get_available_packet_count() < 1:
		return null
	var pkt = socket.get_packet()
	if socket.was_string_packet():
		return pkt.get_string_from_utf8()
	return bytes_to_var(pkt)


func close(code := 1000, reason := "") -> void:
	socket.close(code, reason)
	last_state = socket.get_ready_state()


func clear() -> void:
	socket = WebSocketPeer.new()
	last_state = socket.get_ready_state()


func get_socket() -> WebSocketPeer:
	return socket


func poll() -> void:
	if socket.get_ready_state() != socket.STATE_CLOSED:
		socket.poll()
	var state = socket.get_ready_state()
	if last_state != state:
		last_state = state
		if state == socket.STATE_OPEN:
			connected_to_server.emit()
		elif state == socket.STATE_CLOSED:
			connection_closed.emit()
	while socket.get_ready_state() == socket.STATE_OPEN and socket.get_available_packet_count():
		message_received.emit(get_message())

func construct_init_msg():
	var init_obj = {
		"name":"",
		"fuck":"tou",
		"initialise":true
	}
	return JSON.stringify(init_obj)

func _ready():
	connect_to_url(base_url)
	await connected_to_server
	send(construct_init_msg())

func _process(delta):
	poll()
