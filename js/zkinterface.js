var zkif = require('./zkinterface_generated').zkinterface;
var fb = require('flatbuffers').flatbuffers;

/** Finish and add a size prefix. (monkey-patch missing feature) */
fb.Builder.prototype.finishSizePrefixed = function(root_table, opt_file_identifier) {
    this.finish(root_table, opt_file_identifier);
    this.addInt32(this.offset());
    this.bb.setPosition(this.space);
}

/** Format a zkInterface message into a buffer (Uint8Array). */
zkif.finishMessage = function(builder, message_type, message_offset) {
    zkif.Root.startRoot(builder);
    zkif.Root.addMessageType(builder, message_type);
    zkif.Root.addMessage(builder, message_offset);
    var root = zkif.Root.endRoot(builder);
    builder.finishSizePrefixed(root, "zkif");
    return builder.asUint8Array();
}

/** Parse a zkInterface message from a buffer (Uint8Array) */
zkif.parseMessage = function(buffer) {
    var without_prefix = buffer.subarray(4);
    var root = zkif.Root.getRootAsRoot(new fb.ByteBuffer(without_prefix));
    return root;
}

/** Convert a JS number to a zkInterface variable ID (uint64, using flatbuffers.Long). */
zkif.toId = function(num) {
    return new fb.Long(num);
}

/** Convert a zkInterface variable ID to a JS number (53 bits max). */
zkif.fromId = function(long) {
    return long.toFloat64();
}


// Reexport the generated code.
module.exports = zkif;

// Reexport the flatbuffers core.
module.exports.flatbuffers = fb;
module.exports.Builder = fb.Builder;
