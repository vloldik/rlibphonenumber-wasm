import { bench, describe } from 'vitest';
import * as fs from 'fs';
import * as path from 'path';
import { fileURLToPath } from 'url';

import init, {
    parseAndKeepRawInputWithDefaultRegion,
    JsPhoneNumberFormat
} from '../pkg/rlibphonenumber_wasm.js';

import { parsePhoneNumberWithError } from 'libphonenumber-js';
import googleLib from 'google-libphonenumber';
const googlePhoneUtil = googleLib.PhoneNumberUtil.getInstance();
const googleFormat = googleLib.PhoneNumberFormat;

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const wasmPath = path.resolve(__dirname, '../pkg/rlibphonenumber_wasm_bg.wasm');
const nodeBuffer = fs.readFileSync(wasmPath);
const wasmArrayBuffer = new Uint8Array(nodeBuffer.buffer, nodeBuffer.byteOffset, nodeBuffer.length);

await init(wasmArrayBuffer);

describe('Phone Number Parsing & Formatting', () => {
    const TEST_NUMBER = '202-555-0136';
    const TEST_REGION = 'US';


    bench('1. WASM: rlibphonenumber', () => {
        const num = parseAndKeepRawInputWithDefaultRegion(TEST_NUMBER, TEST_REGION);
        num.formatAs(JsPhoneNumberFormat.E164);
        num.free();
    });

    bench('2. Fast JS: libphonenumber-js', () => {
        const num = parsePhoneNumberWithError(TEST_NUMBER, TEST_REGION);
        num.format('E.164');
    });

    bench('3. Google Orig: google-libphonenumber', () => {
        const num = googlePhoneUtil.parse(TEST_NUMBER, TEST_REGION);
        googlePhoneUtil.format(num, googleFormat.E164);
    });

    bench('1. WASM: rlibphonenumber (isValid)', () => {
        const num = parseAndKeepRawInputWithDefaultRegion(TEST_NUMBER, TEST_REGION);
        num.isValid();
        num.free();
    });

    bench('2. Fast JS: libphonenumber-js (isValid)', () => {
        const num = parsePhoneNumberWithError(TEST_NUMBER, TEST_REGION);
        num.isValid();
    });

    bench('3. Google Orig: google-libphonenumber (isValid)', () => {
        const num = googlePhoneUtil.parse(TEST_NUMBER, TEST_REGION);
        googlePhoneUtil.isValidNumber(num);
    });
});