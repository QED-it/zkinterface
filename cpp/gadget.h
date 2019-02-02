#ifndef GADGET_H
#define GADGET_H

#ifdef __cplusplus
extern "C" {
#endif


typedef bool (*gadget_callback_t)(
        void *context,
        unsigned char *response
);

bool gadget_request(
        unsigned char *request,

        gadget_callback_t result_stream_callback,
        void *result_stream_context,

        gadget_callback_t response_callback,
        void *response_context
);


#ifdef __cplusplus
} // extern "C"
#endif

#endif //GADGET_H
