function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_55538483de6e3abe: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_Number_f257194b7002d6f9: function(arg0) {
            const ret = Number(arg0);
            return ret;
        },
        __wbg___wbindgen_boolean_get_fe2a24fdfdb4064f: function(arg0) {
            const v = arg0;
            const ret = typeof(v) === 'boolean' ? v : undefined;
            return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
        },
        __wbg___wbindgen_debug_string_d89627202d0155b7: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_in_fe3eb6a509f75744: function(arg0, arg1) {
            const ret = arg0 in arg1;
            return ret;
        },
        __wbg___wbindgen_is_function_2a95406423ea8626: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_object_59a002e76b059312: function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_string_624d5244bb2bc87c: function(arg0) {
            const ret = typeof(arg0) === 'string';
            return ret;
        },
        __wbg___wbindgen_is_undefined_87a3a837f331fef5: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_jsval_eq_eedd705f9f2a4f35: function(arg0, arg1) {
            const ret = arg0 === arg1;
            return ret;
        },
        __wbg___wbindgen_jsval_loose_eq_cf851f110c48f9ba: function(arg0, arg1) {
            const ret = arg0 == arg1;
            return ret;
        },
        __wbg___wbindgen_number_get_769f3676dc20c1d7: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_string_get_f1161390414f9b59: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_5549492daedad139: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_fbe69bb076c16bad: function(arg0) {
            arg0._wbg_cb_unref();
        },
        __wbg_abort_b007790bcfd9fff2: function(arg0, arg1) {
            arg0.abort(arg1);
        },
        __wbg_abort_bdf419e9dcbdaeb3: function(arg0) {
            arg0.abort();
        },
        __wbg_accuracy_d1ab49789618a879: function(arg0) {
            const ret = arg0.accuracy;
            return ret;
        },
        __wbg_addEventListener_22cc39177d983010: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);
        }, arguments); },
        __wbg_addEventListener_ee34fcb181ae85b2: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
        }, arguments); },
        __wbg_alert_1b18c7402c943c65: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.alert(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_altKey_fbd30c4040122f9a: function(arg0) {
            const ret = arg0.altKey;
            return ret;
        },
        __wbg_altitudeAccuracy_ea95600e5349d94c: function(arg0, arg1) {
            const ret = arg1.altitudeAccuracy;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg_altitude_09796e367abc34fc: function(arg0, arg1) {
            const ret = arg1.altitude;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg_appendChild_79e0048b1f1b7921: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.appendChild(arg1);
            return ret;
        }, arguments); },
        __wbg_append_7c8e49986ab5288d: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.append(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_arrayBuffer_9f258d017f7107c5: function() { return handleError(function (arg0) {
            const ret = arg0.arrayBuffer();
            return ret;
        }, arguments); },
        __wbg_arrayBuffer_f64a4dc47790c6db: function(arg0) {
            const ret = arg0.arrayBuffer();
            return ret;
        },
        __wbg_body_42a78770fcb42acd: function(arg0) {
            const ret = arg0.body;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_bottom_fdbb52d377b4cf05: function(arg0) {
            const ret = arg0.bottom;
            return ret;
        },
        __wbg_bubbles_eb5e5d845d178a31: function(arg0) {
            const ret = arg0.bubbles;
            return ret;
        },
        __wbg_buffered_19a6b6a57f29d6c3: function(arg0) {
            const ret = arg0.buffered;
            return ret;
        },
        __wbg_cache_key_581e6d43e117266a: function(arg0) {
            const ret = arg0.__yew_subtree_cache_key;
            return isLikeNone(ret) ? 0x100000001 : (ret) >>> 0;
        },
        __wbg_call_6ae20895a60069a2: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_call_8f5d7bb070283508: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_cancelAnimationFrame_0d2a6a6409c37763: function() { return handleError(function (arg0, arg1) {
            arg0.cancelAnimationFrame(arg1);
        }, arguments); },
        __wbg_cancelBubble_26aa93f493097d64: function(arg0) {
            const ret = arg0.cancelBubble;
            return ret;
        },
        __wbg_changedTouches_66d1b01442bf6cb4: function(arg0) {
            const ret = arg0.changedTouches;
            return ret;
        },
        __wbg_childNodes_0ad633dde5fc01d0: function(arg0) {
            const ret = arg0.childNodes;
            return ret;
        },
        __wbg_clearInterval_16e8cbbce92291d0: function(arg0) {
            const ret = clearInterval(arg0);
            return ret;
        },
        __wbg_clearTimeout_113b1cde814ec762: function(arg0) {
            const ret = clearTimeout(arg0);
            return ret;
        },
        __wbg_clearTimeout_6b8d9a38b9263d65: function(arg0) {
            const ret = clearTimeout(arg0);
            return ret;
        },
        __wbg_clearWatch_87cf4782675b7a7d: function(arg0, arg1) {
            arg0.clearWatch(arg1);
        },
        __wbg_clientHeight_a088c0a046303c1e: function(arg0) {
            const ret = arg0.clientHeight;
            return ret;
        },
        __wbg_clientWidth_489775f176e76aa6: function(arg0) {
            const ret = arg0.clientWidth;
            return ret;
        },
        __wbg_clientX_4136e615bf7e2a7c: function(arg0) {
            const ret = arg0.clientX;
            return ret;
        },
        __wbg_clientX_ebc7e3f31a9492bd: function(arg0) {
            const ret = arg0.clientX;
            return ret;
        },
        __wbg_clientY_26dcab94f3942474: function(arg0) {
            const ret = arg0.clientY;
            return ret;
        },
        __wbg_clientY_c39d37c3a25ed988: function(arg0) {
            const ret = arg0.clientY;
            return ret;
        },
        __wbg_clipboardData_241a009790ca3034: function(arg0) {
            const ret = arg0.clipboardData;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_clipboard_491686e5ce3c41a4: function(arg0) {
            const ret = arg0.clipboard;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_cloneNode_d7b1e5ec312a7faf: function() { return handleError(function (arg0) {
            const ret = arg0.cloneNode();
            return ret;
        }, arguments); },
        __wbg_close_1bf0654059764e94: function() { return handleError(function (arg0) {
            arg0.close();
        }, arguments); },
        __wbg_composedPath_058226d41776bbad: function(arg0) {
            const ret = arg0.composedPath();
            return ret;
        },
        __wbg_contains_2e46bcbe76a49ac2: function(arg0, arg1) {
            const ret = arg0.contains(arg1);
            return ret;
        },
        __wbg_contentRect_1a5b99891d25c023: function(arg0) {
            const ret = arg0.contentRect;
            return ret;
        },
        __wbg_coords_d8108ffe6c455187: function(arg0) {
            const ret = arg0.coords;
            return ret;
        },
        __wbg_createElementNS_f3714b3aee0a62f3: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            const ret = arg0.createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            return ret;
        }, arguments); },
        __wbg_createElement_a8dcfa25dbf80c51: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_createTask_2e55ab0673ae81d8: function() { return handleError(function (arg0, arg1) {
            const ret = console.createTask(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_createTextNode_9d7d2fed8d685efa: function(arg0, arg1, arg2) {
            const ret = arg0.createTextNode(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_crypto_38df2bab126b63dc: function(arg0) {
            const ret = arg0.crypto;
            return ret;
        },
        __wbg_ctrlKey_81931b7aea899650: function(arg0) {
            const ret = arg0.ctrlKey;
            return ret;
        },
        __wbg_currentTime_64ec47174225e740: function(arg0) {
            const ret = arg0.currentTime;
            return ret;
        },
        __wbg_dataTransfer_89aea0b8352d3dd3: function(arg0) {
            const ret = arg0.dataTransfer;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_data_7de671a92a650aba: function(arg0) {
            const ret = arg0.data;
            return ret;
        },
        __wbg_debug_61e14ffba79c6807: function(arg0, arg1, arg2, arg3) {
            console.debug(arg0, arg1, arg2, arg3);
        },
        __wbg_disconnect_6cc70c5f1dd86a91: function(arg0) {
            arg0.disconnect();
        },
        __wbg_disconnect_d841e2be4967e598: function(arg0) {
            arg0.disconnect();
        },
        __wbg_documentElement_c375315afd6a4740: function(arg0) {
            const ret = arg0.documentElement;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_document_cf512e4e2300751d: function(arg0) {
            const ret = arg0.document;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_done_19f92cb1f8738aba: function(arg0) {
            const ret = arg0.done;
            return ret;
        },
        __wbg_duration_5bff0f1c554b5aa9: function(arg0) {
            const ret = arg0.duration;
            return ret;
        },
        __wbg_end_774744757c76c57d: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.end(arg1 >>> 0);
            return ret;
        }, arguments); },
        __wbg_entries_28ed7cb892e12eff: function(arg0) {
            const ret = Object.entries(arg0);
            return ret;
        },
        __wbg_error_4c20fd6d19d38f03: function(arg0, arg1) {
            var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
            wasm.__wbindgen_free(arg0, arg1 * 4, 4);
            console.error(...v0);
        },
        __wbg_error_9126ccb9e84e16ff: function(arg0, arg1, arg2, arg3) {
            console.error(arg0, arg1, arg2, arg3);
        },
        __wbg_error_a6fa202b58aa1cd3: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_error_de6b86e598505246: function(arg0) {
            console.error(arg0);
        },
        __wbg_fetch_3f39346b50886803: function(arg0, arg1) {
            const ret = arg0.fetch(arg1);
            return ret;
        },
        __wbg_fetch_9dad4fe911207b37: function(arg0) {
            const ret = fetch(arg0);
            return ret;
        },
        __wbg_files_dad88a18a0646065: function(arg0) {
            const ret = arg0.files;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_from_45cebbf5e49a6ac6: function(arg0) {
            const ret = Array.from(arg0);
            return ret;
        },
        __wbg_geolocation_79f684a5630a816c: function() { return handleError(function (arg0) {
            const ret = arg0.geolocation;
            return ret;
        }, arguments); },
        __wbg_getAttribute_94c78997e59ce800: function(arg0, arg1, arg2, arg3) {
            const ret = arg1.getAttribute(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_getCurrentPosition_901773cdbc4fe7cd: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.getCurrentPosition(arg1, arg2, arg3);
        }, arguments); },
        __wbg_getData_ec6498e981620fc7: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg1.getData(getStringFromWasm0(arg2, arg3));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_getElementsByTagName_2c218f1a8fb9ed3b: function(arg0, arg1, arg2) {
            const ret = arg0.getElementsByTagName(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_getItem_370efb66c7950f06: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg1.getItem(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_getRandomValues_c44a50d8cfdaebeb: function() { return handleError(function (arg0, arg1) {
            arg0.getRandomValues(arg1);
        }, arguments); },
        __wbg_getType_c2489fdea74a5ef1: function(arg0, arg1, arg2) {
            const ret = arg0.getType(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_get_197d34851780fe91: function(arg0, arg1, arg2, arg3) {
            const ret = arg1.get(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_get_6eadbda4395687cb: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_get_94f5fc088edd3138: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_get_a50328e7325d7f9b: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_ff5f1fb220233477: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_unchecked_7c6bbabf5b0b1fbf: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_get_with_ref_key_6412cf3094599694: function(arg0, arg1) {
            const ret = arg0[arg1];
            return ret;
        },
        __wbg_has_3f87d148146a0f4e: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.has(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_hash_43bfda7f0e2a128f: function(arg0, arg1) {
            const ret = arg1.hash;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_hash_7ae903ee70c3dd41: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.hash;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_headers_6ccffabdaab0d021: function(arg0) {
            const ret = arg0.headers;
            return ret;
        },
        __wbg_heading_e0cb11f30475bdae: function(arg0, arg1) {
            const ret = arg1.heading;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg_height_6324f7f0eb2ef72c: function(arg0) {
            const ret = arg0.height;
            return ret;
        },
        __wbg_history_304baab31767c8be: function() { return handleError(function (arg0) {
            const ret = arg0.history;
            return ret;
        }, arguments); },
        __wbg_host_c563222bae213767: function(arg0) {
            const ret = arg0.host;
            return ret;
        },
        __wbg_host_fdcb41d66cd3e943: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.host;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_hostname_7024ec4140c88645: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.hostname;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_href_55909e641b55be73: function(arg0, arg1) {
            const ret = arg1.href;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_href_ec3def677e7c4414: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.href;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_info_4a899082bb96facf: function(arg0, arg1, arg2, arg3) {
            console.info(arg0, arg1, arg2, arg3);
        },
        __wbg_innerHeight_0ea00febf4940b36: function() { return handleError(function (arg0) {
            const ret = arg0.innerHeight;
            return ret;
        }, arguments); },
        __wbg_innerWidth_8b227093ee989fa1: function() { return handleError(function (arg0) {
            const ret = arg0.innerWidth;
            return ret;
        }, arguments); },
        __wbg_insertBefore_b7d9275f2d729de5: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.insertBefore(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_instanceof_ArrayBuffer_8d855993947fc3a2: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Blob_0ba6040bc29f038a: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Blob;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_ClipboardItem_f6c049d7ed7ae320: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ClipboardItem;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Element_60d1572f735fb2ce: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Element;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Error_e14ad3dc04e9f18c: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Error;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_EventTarget_b727a8087d258184: function(arg0) {
            let result;
            try {
                result = arg0 instanceof EventTarget;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlElement_78785ade06923935: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlLinkElement_2a407e30e0e86d09: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLLinkElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlMediaElement_0dbd3c73d2c62351: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLMediaElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Node_c72895807e8b83e6: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Node;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_PermissionStatus_51386a1eb97f0a72: function(arg0) {
            let result;
            try {
                result = arg0 instanceof PermissionStatus;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Response_fece7eabbcaca4c3: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Response;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_ShadowRoot_aa005ef99a0ca50b: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ShadowRoot;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Uint8Array_ce24d58a5f4bdcc3: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_0bf605aca084fcde: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_2fa8d9c2d5b6104a: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_intersectionRatio_026bad35aaed1171: function(arg0) {
            const ret = arg0.intersectionRatio;
            return ret;
        },
        __wbg_isSafeInteger_1dfae065cbfe1915: function(arg0) {
            const ret = Number.isSafeInteger(arg0);
            return ret;
        },
        __wbg_is_8d0dad0e07b754c5: function(arg0, arg1) {
            const ret = Object.is(arg0, arg1);
            return ret;
        },
        __wbg_item_6344995201c32171: function(arg0, arg1) {
            const ret = arg0.item(arg1 >>> 0);
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_item_7da69d673a9dbfc8: function(arg0, arg1) {
            const ret = arg0.item(arg1 >>> 0);
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_iterator_54661826e186eb6a: function() {
            const ret = Symbol.iterator;
            return ret;
        },
        __wbg_key_75d93864da2a4ec7: function(arg0, arg1) {
            const ret = arg1.key;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_key_b01bf4276ad929a2: function(arg0, arg1) {
            const ret = arg1.key;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_lastChild_69d7f331358bca60: function(arg0) {
            const ret = arg0.lastChild;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_latitude_3ad2831c573d7e68: function(arg0) {
            const ret = arg0.latitude;
            return ret;
        },
        __wbg_left_3b637e6527576a90: function(arg0) {
            const ret = arg0.left;
            return ret;
        },
        __wbg_length_4c5cd127b105dd20: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_7f72241da7ac742a: function() { return handleError(function (arg0) {
            const ret = arg0.length;
            return ret;
        }, arguments); },
        __wbg_length_85dfb54ae1d3ef8d: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_e6e1633fbea6cfa9: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_fae3e439140f48a4: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_listener_id_fd5d191d59511336: function(arg0) {
            const ret = arg0.__yew_listener_id;
            return isLikeNone(ret) ? 0x100000001 : (ret) >>> 0;
        },
        __wbg_localStorage_6af88123156fd954: function() { return handleError(function (arg0) {
            const ret = arg0.localStorage;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_location_639bffbf5fe10651: function(arg0) {
            const ret = arg0.location;
            return ret;
        },
        __wbg_log_da87efab6aec47d3: function(arg0, arg1, arg2, arg3) {
            console.log(arg0, arg1, arg2, arg3);
        },
        __wbg_longitude_f19ec8a53627f4cc: function(arg0) {
            const ret = arg0.longitude;
            return ret;
        },
        __wbg_message_613f79663a57a145: function(arg0) {
            const ret = arg0.message;
            return ret;
        },
        __wbg_metaKey_5ad20c9c8d673f9c: function(arg0) {
            const ret = arg0.metaKey;
            return ret;
        },
        __wbg_msCrypto_bd5a034af96bcba6: function(arg0) {
            const ret = arg0.msCrypto;
            return ret;
        },
        __wbg_muted_d6d729dc4c368360: function(arg0) {
            const ret = arg0.muted;
            return ret;
        },
        __wbg_name_13eb211eafa5eae9: function(arg0, arg1) {
            const ret = arg1.name;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_name_3aa8b7545ecd9f40: function(arg0) {
            const ret = arg0.name;
            return ret;
        },
        __wbg_namespaceURI_2d8879ce91545d15: function(arg0, arg1) {
            const ret = arg1.namespaceURI;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_navigator_47164ffacf3edc06: function(arg0) {
            const ret = arg0.navigator;
            return ret;
        },
        __wbg_navigator_a1abf8a959c14f80: function(arg0) {
            const ret = arg0.navigator;
            return ret;
        },
        __wbg_new_0_e649c99e7382313f: function() {
            const ret = new Date();
            return ret;
        },
        __wbg_new_11f863758d68139f: function() { return handleError(function (arg0) {
            const ret = new ResizeObserver(arg0);
            return ret;
        }, arguments); },
        __wbg_new_1d5ac9ee39b9cacb: function() { return handleError(function (arg0) {
            const ret = new ClipboardItem(arg0);
            return ret;
        }, arguments); },
        __wbg_new_1d96678aaacca32e: function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        },
        __wbg_new_210ef5849ab6cf48: function() { return handleError(function () {
            const ret = new Headers();
            return ret;
        }, arguments); },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_new_270e5eb94e5160f6: function() { return handleError(function (arg0) {
            const ret = new IntersectionObserver(arg0);
            return ret;
        }, arguments); },
        __wbg_new_4370be21fa2b2f80: function() {
            const ret = new Array();
            return ret;
        },
        __wbg_new_48e1d86cfd30c8e7: function() {
            const ret = new Object();
            return ret;
        },
        __wbg_new_5c9ffcb4c283a499: function() { return handleError(function (arg0, arg1) {
            const ret = new MouseEvent(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_new_69642b0f6c3151cc: function() { return handleError(function (arg0, arg1) {
            const ret = new WebSocket(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_new_b906fdbc60e83470: function() { return handleError(function (arg0, arg1) {
            const ret = new URL(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_new_ce17f0bcfcc7b8ef: function() { return handleError(function () {
            const ret = new AbortController();
            return ret;
        }, arguments); },
        __wbg_new_d51bf22e953dcfd1: function(arg0) {
            const ret = new Date(arg0);
            return ret;
        },
        __wbg_new_from_slice_0bc58e36f82a1b50: function(arg0, arg1) {
            const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_new_with_base_7d8ccfd7797e6ff4: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = new URL(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
            return ret;
        }, arguments); },
        __wbg_new_with_length_0f3108b57e05ed7c: function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return ret;
        },
        __wbg_new_with_str_and_init_cb3df438bf62964e: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = new Request(getStringFromWasm0(arg0, arg1), arg2);
            return ret;
        }, arguments); },
        __wbg_new_with_str_f3d00d5938d7b751: function() { return handleError(function (arg0, arg1) {
            const ret = new URLSearchParams(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_new_with_str_sequence_d1f1d1430bc0f626: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = new WebSocket(getStringFromWasm0(arg0, arg1), arg2);
            return ret;
        }, arguments); },
        __wbg_new_with_u8_array_sequence_and_options_93acf864134c05b7: function() { return handleError(function (arg0, arg1) {
            const ret = new Blob(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_nextSibling_b5ea0d24290942af: function(arg0) {
            const ret = arg0.nextSibling;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_next_55d835fe0ab5b3e7: function(arg0) {
            const ret = arg0.next;
            return ret;
        },
        __wbg_next_e34cfb9df1518d7c: function() { return handleError(function (arg0) {
            const ret = arg0.next();
            return ret;
        }, arguments); },
        __wbg_node_84ea875411254db1: function(arg0) {
            const ret = arg0.node;
            return ret;
        },
        __wbg_now_46736a527d2e74e7: function() {
            const ret = Date.now();
            return ret;
        },
        __wbg_observe_f45c570339e85121: function(arg0, arg1) {
            arg0.observe(arg1);
        },
        __wbg_observe_fec7019828d7ba78: function(arg0, arg1) {
            arg0.observe(arg1);
        },
        __wbg_of_a96e15740cdace88: function(arg0) {
            const ret = Array.of(arg0);
            return ret;
        },
        __wbg_origin_ce0d5156c01dcd67: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.origin;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_outerHTML_bf1929ba3554206f: function(arg0, arg1) {
            const ret = arg1.outerHTML;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_pageXOffset_e74ef6f5dbb63229: function() { return handleError(function (arg0) {
            const ret = arg0.pageXOffset;
            return ret;
        }, arguments); },
        __wbg_pageYOffset_3319b90ae8ff40b9: function() { return handleError(function (arg0) {
            const ret = arg0.pageYOffset;
            return ret;
        }, arguments); },
        __wbg_parentElement_90ebc389e4feec98: function(arg0) {
            const ret = arg0.parentElement;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_parentNode_762a8626d5cc08d9: function(arg0) {
            const ret = arg0.parentNode;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_pathname_5a9c966225d90655: function(arg0, arg1) {
            const ret = arg1.pathname;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_pathname_f1d35181cabf06f5: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.pathname;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_pause_7593b30e1368764d: function() { return handleError(function (arg0) {
            arg0.pause();
        }, arguments); },
        __wbg_paused_809294a1da2b8c18: function(arg0) {
            const ret = arg0.paused;
            return ret;
        },
        __wbg_permissions_dda96cc397a4f8c3: function() { return handleError(function (arg0) {
            const ret = arg0.permissions;
            return ret;
        }, arguments); },
        __wbg_play_7b1e3745d94a9890: function() { return handleError(function (arg0) {
            const ret = arg0.play();
            return ret;
        }, arguments); },
        __wbg_port_f849b64e501950ab: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.port;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_preventDefault_a85164025505c2f9: function(arg0) {
            arg0.preventDefault();
        },
        __wbg_process_44c7a14e11e9f69e: function(arg0) {
            const ret = arg0.process;
            return ret;
        },
        __wbg_protocol_19d44b8be55114d9: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.protocol;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_prototypesetcall_3875d54d12ef2eec: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        },
        __wbg_pushState_e06769078a41fe94: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.pushState(arg1, getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        }, arguments); },
        __wbg_push_d0006a37f9fcda6d: function(arg0, arg1) {
            const ret = arg0.push(arg1);
            return ret;
        },
        __wbg_querySelector_57e670971375e069: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_query_3d9389524692d33e: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.query(arg1);
            return ret;
        }, arguments); },
        __wbg_queueMicrotask_8868365114fe23b5: function(arg0) {
            queueMicrotask(arg0);
        },
        __wbg_queueMicrotask_cfc5a0e62f9ebdbe: function(arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        },
        __wbg_randomFillSync_6c25eac9869eb53c: function() { return handleError(function (arg0, arg1) {
            arg0.randomFillSync(arg1);
        }, arguments); },
        __wbg_readText_17324a9e9fbf750f: function(arg0) {
            const ret = arg0.readText();
            return ret;
        },
        __wbg_read_cefb45b4cc1300c4: function(arg0) {
            const ret = arg0.read();
            return ret;
        },
        __wbg_readyState_a08d25cc57214030: function(arg0) {
            const ret = arg0.readyState;
            return ret;
        },
        __wbg_removeAttribute_a2d8bdae9c65b7af: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.removeAttribute(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_removeChild_d4e08300f4694161: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.removeChild(arg1);
            return ret;
        }, arguments); },
        __wbg_removeEventListener_05045076192ed5e7: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4 !== 0);
        }, arguments); },
        __wbg_removeItem_d18a258184a546a6: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.removeItem(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_replaceState_ac4502ca173046c9: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.replaceState(arg1, getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        }, arguments); },
        __wbg_requestAnimationFrame_3b4101343dfc745b: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.requestAnimationFrame(arg1);
            return ret;
        }, arguments); },
        __wbg_require_b4edbdcf3e2a1ef0: function() { return handleError(function () {
            const ret = module.require;
            return ret;
        }, arguments); },
        __wbg_resolve_d8059bc113e215bf: function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        },
        __wbg_right_23f843908f791fbb: function(arg0) {
            const ret = arg0.right;
            return ret;
        },
        __wbg_run_73e1363df4d8e653: function(arg0, arg1, arg2) {
            try {
                var state0 = {a: arg1, b: arg2};
                var cb0 = () => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen__convert__closures_____invoke__h544094067ea143d5(a, state0.b, );
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = arg0.run(cb0);
                return ret;
            } finally {
                state0.a = 0;
            }
        },
        __wbg_scrollHeight_826422356740f5c6: function(arg0) {
            const ret = arg0.scrollHeight;
            return ret;
        },
        __wbg_scrollLeft_5f3950752da7f3b6: function(arg0) {
            const ret = arg0.scrollLeft;
            return ret;
        },
        __wbg_scrollTop_0c074439b8359388: function(arg0) {
            const ret = arg0.scrollTop;
            return ret;
        },
        __wbg_scrollTop_cf925e21bf7a9e84: function(arg0) {
            const ret = arg0.scrollTop;
            return ret;
        },
        __wbg_search_0da984ff6207e002: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.search;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_search_a8db99ec94d5c914: function(arg0, arg1) {
            const ret = arg1.search;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_send_73e9cb70b2a23e05: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.send(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_send_da543a379e952bc6: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.send(getArrayU8FromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_sessionStorage_97b9f263831866d3: function() { return handleError(function (arg0) {
            const ret = arg0.sessionStorage;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_setAttribute_981e9c312a132ede: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setData_8e906f4a6ab48d5c: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setData(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setInterval_84b64f01452a246e: function() { return handleError(function (arg0, arg1) {
            const ret = setInterval(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_setItem_fd14fd8704eedc89: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setItem(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setProperty_f0f7114b876cd68a: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setTimeout_a89fa3173dd1b518: function(arg0, arg1) {
            const ret = setTimeout(arg0, arg1);
            return ret;
        },
        __wbg_setTimeout_ef24d2fc3ad97385: function() { return handleError(function (arg0, arg1) {
            const ret = setTimeout(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_setTimeout_f757f00851f76c42: function(arg0, arg1) {
            const ret = setTimeout(arg0, arg1);
            return ret;
        },
        __wbg_set_6be42768c690e380: function(arg0, arg1, arg2) {
            arg0[arg1] = arg2;
        },
        __wbg_set_991082a7a49971cf: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(arg0, arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_set_binaryType_0675f0e51c055ca8: function(arg0, arg1) {
            arg0.binaryType = __wbindgen_enum_BinaryType[arg1];
        },
        __wbg_set_body_e2cf9537a2f3e0be: function(arg0, arg1) {
            arg0.body = arg1;
        },
        __wbg_set_cache_542e710bfd7aa57a: function(arg0, arg1) {
            arg0.cache = __wbindgen_enum_RequestCache[arg1];
        },
        __wbg_set_cache_key_65a529cd1f95fc20: function(arg0, arg1) {
            arg0.__yew_subtree_cache_key = arg1 >>> 0;
        },
        __wbg_set_capture_6b6ae72bd7165dd4: function(arg0, arg1) {
            arg0.capture = arg1 !== 0;
        },
        __wbg_set_checked_af7ab5402ef2d0c9: function(arg0, arg1) {
            arg0.checked = arg1 !== 0;
        },
        __wbg_set_controls_5967e56de40ba20e: function(arg0, arg1) {
            arg0.controls = arg1 !== 0;
        },
        __wbg_set_credentials_5838a4909b379d8e: function(arg0, arg1) {
            arg0.credentials = __wbindgen_enum_RequestCredentials[arg1];
        },
        __wbg_set_currentTime_db2bd72283f4a8f9: function(arg0, arg1) {
            arg0.currentTime = arg1;
        },
        __wbg_set_defaultValue_b01ce7be51404827: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.defaultValue = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_hash_75101de69065074c: function(arg0, arg1, arg2) {
            arg0.hash = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_hash_b8adc17170068bfb: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.hash = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_headers_22d4b01224273a83: function(arg0, arg1) {
            arg0.headers = arg1;
        },
        __wbg_set_href_34d24a4ac1dd58ad: function(arg0, arg1, arg2) {
            arg0.href = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_innerHTML_e2a0c521ebf9b8f3: function(arg0, arg1, arg2) {
            arg0.innerHTML = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_listener_id_bdd6845f6cb449dd: function(arg0, arg1) {
            arg0.__yew_listener_id = arg1 >>> 0;
        },
        __wbg_set_method_4a4ab3faba8a018c: function(arg0, arg1, arg2) {
            arg0.method = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_mode_7b856ab49b64c0db: function(arg0, arg1) {
            arg0.mode = __wbindgen_enum_RequestMode[arg1];
        },
        __wbg_set_muted_e9d1d7bb33c6143d: function(arg0, arg1) {
            arg0.muted = arg1 !== 0;
        },
        __wbg_set_nodeValue_36fce4b3dd75fc69: function(arg0, arg1, arg2) {
            arg0.nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_once_3f8e98ae18cf7525: function(arg0, arg1) {
            arg0.once = arg1 !== 0;
        },
        __wbg_set_onchange_b639b8a6284ea833: function(arg0, arg1) {
            arg0.onchange = arg1;
        },
        __wbg_set_onclose_f791ef701be808a0: function(arg0, arg1) {
            arg0.onclose = arg1;
        },
        __wbg_set_onerror_e23002e9224d353b: function(arg0, arg1) {
            arg0.onerror = arg1;
        },
        __wbg_set_onmessage_d2fe701a9ce80846: function(arg0, arg1) {
            arg0.onmessage = arg1;
        },
        __wbg_set_onopen_0556381d0db30cbb: function(arg0, arg1) {
            arg0.onopen = arg1;
        },
        __wbg_set_passive_9e23184a10341acd: function(arg0, arg1) {
            arg0.passive = arg1 !== 0;
        },
        __wbg_set_rel_568b8102414d8523: function(arg0, arg1, arg2) {
            arg0.rel = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_returnValue_688d72502a75c52c: function(arg0, arg1, arg2) {
            arg0.returnValue = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_scrollTop_1f641ec7b36ccd6f: function(arg0, arg1) {
            arg0.scrollTop = arg1;
        },
        __wbg_set_search_f793f9b7d0fd753c: function(arg0, arg1, arg2) {
            arg0.search = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_signal_cd4528432ab8fe0b: function(arg0, arg1) {
            arg0.signal = arg1;
        },
        __wbg_set_src_a83b6259b44cd636: function(arg0, arg1, arg2) {
            arg0.src = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_subtree_id_7234f128830a05c9: function(arg0, arg1) {
            arg0.__yew_subtree_id = arg1 >>> 0;
        },
        __wbg_set_title_6ccaecd1469532e8: function(arg0, arg1, arg2) {
            arg0.title = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_type_dbeec016841a2a68: function(arg0, arg1, arg2) {
            arg0.type = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_type_e8ab4a5ab35ebb6f: function(arg0, arg1, arg2) {
            arg0.type = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_value_7f45d2ec472e7855: function(arg0, arg1, arg2) {
            arg0.value = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_value_c2e2ca2c9aec94f1: function(arg0, arg1, arg2) {
            arg0.value = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_volume_cd075bb5d88d4a37: function(arg0, arg1) {
            arg0.volume = arg1;
        },
        __wbg_shiftKey_f53c305e9069c8d4: function(arg0) {
            const ret = arg0.shiftKey;
            return ret;
        },
        __wbg_signal_6740ecf9bc372e29: function(arg0) {
            const ret = arg0.signal;
            return ret;
        },
        __wbg_slice_5446e93d57fd40ad: function(arg0, arg1) {
            const ret = arg1.slice();
            const ptr1 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_speed_c8d4a5c20f4e32e9: function(arg0, arg1) {
            const ret = arg1.speed;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_start_885bd9745e8ee444: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.start(arg1 >>> 0);
            return ret;
        }, arguments); },
        __wbg_state_2bb480c88adcea79: function() { return handleError(function (arg0) {
            const ret = arg0.state;
            return ret;
        }, arguments); },
        __wbg_state_d3f537e9f999ab37: function(arg0) {
            const ret = arg0.state;
            return (__wbindgen_enum_PermissionState.indexOf(ret) + 1 || 4) - 1;
        },
        __wbg_static_accessor_GLOBAL_8dfb7f5e26ebe523: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_941154efc8395cdd: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_58dac9af822f561f: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_ee64f0b3d8354c0b: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_status_1ae443dc56281de7: function(arg0) {
            const ret = arg0.status;
            return ret;
        },
        __wbg_stringify_b67e2c8c60b93f69: function() { return handleError(function (arg0) {
            const ret = JSON.stringify(arg0);
            return ret;
        }, arguments); },
        __wbg_style_61d57921820e35b2: function(arg0) {
            const ret = arg0.style;
            return ret;
        },
        __wbg_subarray_035d32bb24a7d55d: function(arg0, arg1, arg2) {
            const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
            return ret;
        },
        __wbg_subtree_id_32413ad1d938625a: function(arg0) {
            const ret = arg0.__yew_subtree_id;
            return isLikeNone(ret) ? 0x100000001 : (ret) >>> 0;
        },
        __wbg_target_c2d80ee4d3287cbb: function(arg0) {
            const ret = arg0.target;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_target_db0ac901b1046b50: function(arg0) {
            const ret = arg0.target;
            return ret;
        },
        __wbg_textContent_c1c48d6ac47606b1: function(arg0, arg1) {
            const ret = arg1.textContent;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_then_0150352e4ad20344: function(arg0, arg1, arg2) {
            const ret = arg0.then(arg1, arg2);
            return ret;
        },
        __wbg_then_5160486c67ddb98a: function(arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        },
        __wbg_then_60ee697adfaa74bb: function(arg0, arg1, arg2) {
            const ret = arg0.then(arg1, arg2);
            return ret;
        },
        __wbg_timestamp_1be3131ce8425e7b: function(arg0) {
            const ret = arg0.timestamp;
            return ret;
        },
        __wbg_title_59537e706c75850f: function(arg0, arg1) {
            const ret = arg1.title;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_toLocaleTimeString_08701db10a0edf8c: function(arg0, arg1, arg2) {
            const ret = arg0.toLocaleTimeString(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_toString_38b993ab7a0a47a0: function(arg0) {
            const ret = arg0.toString();
            return ret;
        },
        __wbg_toString_553b5f6e95e3e41b: function(arg0) {
            const ret = arg0.toString();
            return ret;
        },
        __wbg_top_7190df445227226f: function(arg0) {
            const ret = arg0.top;
            return ret;
        },
        __wbg_touches_2617b6114abee3ce: function(arg0) {
            const ret = arg0.touches;
            return ret;
        },
        __wbg_types_3852ece84fc1899b: function(arg0) {
            const ret = arg0.types;
            return ret;
        },
        __wbg_url_c6d54634d7005dd1: function(arg0, arg1) {
            const ret = arg1.url;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_5f8653fa438e0c94: function(arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_c3b0708d699ea640: function(arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_d5b248ce8419bd1b: function(arg0) {
            const ret = arg0.value;
            return ret;
        },
        __wbg_versions_276b2795b1c6a219: function(arg0) {
            const ret = arg0.versions;
            return ret;
        },
        __wbg_volume_8f828b4cc53c2e97: function(arg0) {
            const ret = arg0.volume;
            return ret;
        },
        __wbg_warn_443d7dc3883e06f8: function(arg0, arg1, arg2, arg3) {
            console.warn(arg0, arg1, arg2, arg3);
        },
        __wbg_watchPosition_dd0acbe82f59922b: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg0.watchPosition(arg1, arg2, arg3);
            return ret;
        }, arguments); },
        __wbg_width_636a83de4d28badb: function(arg0) {
            const ret = arg0.width;
            return ret;
        },
        __wbg_writeText_cbf28480f347ce7d: function(arg0, arg1, arg2) {
            const ret = arg0.writeText(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_write_56dbeb5007068582: function(arg0, arg1) {
            const ret = arg0.write(arg1);
            return ret;
        },
        __wbg_x_3f75df9d2e78b04d: function(arg0) {
            const ret = arg0.x;
            return ret;
        },
        __wbg_y_631fea1d10bd951f: function(arg0) {
            const ret = arg0.y;
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 4325, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h8dcfa486360e6668);
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 4328, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h7bbd2a4beb0ccca5);
            return ret;
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 5110, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__hd947306d0702438d);
            return ret;
        },
        __wbindgen_cast_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("CloseEvent")], shim_idx: 4984, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h9122f1472802604f);
            return ret;
        },
        __wbindgen_cast_0000000000000005: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Event")], shim_idx: 4986, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h0ab7e7bbed1f7c72);
            return ret;
        },
        __wbindgen_cast_0000000000000006: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("MessageEvent")], shim_idx: 4987, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__ha051db6fdd406d10);
            return ret;
        },
        __wbindgen_cast_0000000000000007: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Position")], shim_idx: 4324, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h7572089ee2f97003);
            return ret;
        },
        __wbindgen_cast_0000000000000008: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("PositionError")], shim_idx: 4326, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__hf538b0766cd0fc54);
            return ret;
        },
        __wbindgen_cast_0000000000000009: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 4891, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures________invoke__h1797ffd9a01207fa);
            return ret;
        },
        __wbindgen_cast_000000000000000a: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 5026, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures________invoke__h3faab4f5eeb5e64c);
            return ret;
        },
        __wbindgen_cast_000000000000000b: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 5057, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures________invoke__hdf455825f09e00e9);
            return ret;
        },
        __wbindgen_cast_000000000000000c: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Vector(NamedExternref("IntersectionObserverEntry")), NamedExternref("IntersectionObserver")], shim_idx: 4327, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__hc776e5725c670310);
            return ret;
        },
        __wbindgen_cast_000000000000000d: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Vector(NamedExternref("ResizeObserverEntry"))], shim_idx: 4323, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__hc30403d49ef1b3a8);
            return ret;
        },
        __wbindgen_cast_000000000000000e: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 3357, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h6a05af6e67065cb7);
            return ret;
        },
        __wbindgen_cast_000000000000000f: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 4985, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__ha8812ab52365d92a);
            return ret;
        },
        __wbindgen_cast_0000000000000010: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000011: function(arg0, arg1) {
            // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
            const ret = getArrayU8FromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000012: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./yew-app_bg.js": import0,
    };
}

function wasm_bindgen__convert__closures_____invoke__h6a05af6e67065cb7(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__h6a05af6e67065cb7(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__ha8812ab52365d92a(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__ha8812ab52365d92a(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__h544094067ea143d5(arg0, arg1) {
    const ret = wasm.wasm_bindgen__convert__closures_____invoke__h544094067ea143d5(arg0, arg1);
    return ret !== 0;
}

function wasm_bindgen__convert__closures_____invoke__h8dcfa486360e6668(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h8dcfa486360e6668(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h7bbd2a4beb0ccca5(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h7bbd2a4beb0ccca5(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h9122f1472802604f(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h9122f1472802604f(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h0ab7e7bbed1f7c72(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h0ab7e7bbed1f7c72(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__ha051db6fdd406d10(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__ha051db6fdd406d10(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h7572089ee2f97003(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h7572089ee2f97003(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__hf538b0766cd0fc54(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__hf538b0766cd0fc54(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures________invoke__h1797ffd9a01207fa(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures________invoke__h1797ffd9a01207fa(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures________invoke__h3faab4f5eeb5e64c(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures________invoke__h3faab4f5eeb5e64c(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures________invoke__hdf455825f09e00e9(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures________invoke__hdf455825f09e00e9(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__hd947306d0702438d(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen__convert__closures_____invoke__hd947306d0702438d(arg0, arg1, arg2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function wasm_bindgen__convert__closures_____invoke__hc30403d49ef1b3a8(arg0, arg1, arg2) {
    const ptr0 = passArrayJsValueToWasm0(arg2, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.wasm_bindgen__convert__closures_____invoke__hc30403d49ef1b3a8(arg0, arg1, ptr0, len0);
}

function wasm_bindgen__convert__closures_____invoke__hc776e5725c670310(arg0, arg1, arg2, arg3) {
    const ptr0 = passArrayJsValueToWasm0(arg2, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.wasm_bindgen__convert__closures_____invoke__hc776e5725c670310(arg0, arg1, ptr0, len0, arg3);
}


const __wbindgen_enum_BinaryType = ["blob", "arraybuffer"];


const __wbindgen_enum_PermissionState = ["granted", "denied", "prompt"];


const __wbindgen_enum_RequestCache = ["default", "no-store", "reload", "no-cache", "force-cache", "only-if-cached"];


const __wbindgen_enum_RequestCredentials = ["omit", "same-origin", "include"];


const __wbindgen_enum_RequestMode = ["same-origin", "no-cors", "cors", "navigate"];

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('yew-app_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
