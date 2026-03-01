import { describe, it, expect, beforeAll } from 'vitest';
import * as fs from 'fs';
import * as path from 'path';
import { fileURLToPath } from 'url';

import init, {
    parseAndKeepRawInputWithDefaultRegion,
    JsPhoneNumberFormat,
    canBeInternationallyDialled,
    JsNumberLengthType
} from '../pkg/rlibphonenumber_wasm.js';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

describe('rlibphonenumber WASM', () => {
    beforeAll(async () => {
        const wasmPath = path.resolve(__dirname, '../pkg/rlibphonenumber_wasm_bg.wasm');
        if (!fs.existsSync(wasmPath)) {
            throw new Error(`CRITICAL: WASM file not found at ${wasmPath}`);
        }
        const buffer = fs.readFileSync(wasmPath);
        await init(buffer);
    });

    it('should parse and format valid US number', () => {
        const number = parseAndKeepRawInputWithDefaultRegion('202-555-0136', 'US');

        expect(number.isValid()).toBe(true);
        expect(number.getRegionCode()).toBe('US');
        expect(number.formatAs(JsPhoneNumberFormat.E164)).toBe('+12025550136');
        expect(number.formatAs(JsPhoneNumberFormat.International)).toBe('+1 202-555-0136');
        expect(number.formatAs(JsPhoneNumberFormat.National)).toBe('(202) 555-0136');
    });

    it('should identify invalid numbers', () => {
        const number = parseAndKeepRawInputWithDefaultRegion('+12089434', 'US');

        expect(number.isValid()).toBe(false);
        expect(number.isPossibleWithReason()).toBe(JsNumberLengthType.IsPossibleLocalOnly);
    });

    it('should throw error on strict garbage', () => {
        expect(() => {
            parseAndKeepRawInputWithDefaultRegion('completely invalid text', 'US');
        }).toThrow();
    });

    it('should verify global functional calls', () => {
        const number = parseAndKeepRawInputWithDefaultRegion('202-555-0136', 'US');
        const canBeDialled = canBeInternationallyDialled(number);

        expect(typeof canBeDialled).toBe('boolean');
    });
});