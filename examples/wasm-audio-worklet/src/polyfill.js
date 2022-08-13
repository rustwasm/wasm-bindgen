if (!globalThis.TextDecoder) {
    globalThis.TextDecoder = class TextDecoder {
        decode(arg) {
            if (typeof arg !== 'undefined') {
                throw Error('TextDecoder stub called');
            } else {
                return '';
            }
        }
    };
}

if (!globalThis.TextEncoder) {
    globalThis.TextEncoder = class TextEncoder {
        encode(arg) {
            if (typeof arg !== 'undefined') {
                throw Error('TextEncoder stub called');
            } else {
                return new Uint8Array(0);
            }
        }
    };
}

export function nop() {
}
