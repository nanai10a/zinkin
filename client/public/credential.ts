import { z } from "zod";

// rome-ignore format:
const base64url = {
  encode: (str: string) =>
    btoa(str).replaceAll("+", "-").replaceAll("/", "_").replaceAll("=", ""),

  encodeBin: (bin: ArrayLike<number> | ArrayBufferLike) =>
    base64url.encode(String.fromCharCode(...Array.from(new Uint8Array(bin)))),

  decode: (str: string) =>
    atob(str.replaceAll("-", "+").replaceAll("_", "/") + "===".slice((str.length + 3) % 4)),

  decodeBin: (str: string) =>
    Uint8Array.from(base64url.decode(str), (c) => c.charCodeAt(0)),
};

export const toRegisterPublicKeyCredential = (cred: Credential) => {
  if (!(cred instanceof PublicKeyCredential)) {
    throw new Error("invalid credential");
  }

  if (!(cred.response instanceof AuthenticatorAttestationResponse)) {
    throw new Error("invalid response");
  }

  const { id, rawId, response, type } = cred;
  const { attestationObject, clientDataJSON } =
    response as AuthenticatorAttestationResponse;

  return {
    id,
    rawId: base64url.encodeBin(rawId),
    response: {
      attestationObject: base64url.encodeBin(attestationObject),
      clientDataJSON: base64url.encodeBin(clientDataJSON),
    },
    type,
    extensions: cred.getClientExtensionResults(),
  };
};

export const toPublicKeyCredential = (cred: Credential) => {
  if (!(cred instanceof PublicKeyCredential)) {
    throw new Error("invalid credential");
  }

  if (!(cred.response instanceof AuthenticatorAssertionResponse)) {
    throw new Error("invalid response");
  }

  const { id, rawId, response, type } = cred;
  const { authenticatorData, clientDataJSON, signature, userHandle } =
    response as AuthenticatorAssertionResponse;

  return {
    id,
    rawId: base64url.encodeBin(rawId),
    response: {
      authenticatorData: base64url.encodeBin(authenticatorData),
      clientDataJSON: base64url.encodeBin(clientDataJSON),
      signature: base64url.encodeBin(signature),
      userHandle: userHandle === null ? null : base64url.encodeBin(userHandle),
    },
    type,
    extensions: cred.getClientExtensionResults(),
  };
};

export const CreateOptions = z.object({
  publicKey: z.object({
    attestation: z
      .union([
        z.literal("none"),
        z.literal("direct"),
        z.literal("enterprise"),
        z.literal("indirect"),
      ])
      .optional(),
    attestationFormats: z.string().array().optional(),
    authenticatorSelection: z
      .object({
        authenticatorAttachment: z
          .union([z.literal("platform"), z.literal("cross-platform")])
          .optional(),
        residentKey: z
          .union([
            z.literal("discouraged"),
            z.literal("preferred"),
            z.literal("required"),
          ])
          .optional(),
        userVerification: z
          .union([
            z.literal("discouraged"),
            z.literal("preferred"),
            z.literal("required"),
          ])
          .optional(),
      })
      .optional(),
    challenge: z.string().transform(base64url.decodeBin),
    excludeCredentials: z
      .object({
        id: z.string().transform(base64url.decodeBin),
        transports: z
          .union([
            z.literal("ble"),
            z.literal("hybrid"),
            z.literal("internal"),
            z.literal("nfc"),
            z.literal("usb"),
          ])
          .array()
          .optional(),
        type: z.literal("public-key"),
      })
      .array()
      .optional(),
    extensions: z.record(z.unknown()).optional(),
    pubKeyCredParams: z
      .object({
        alg: z.number(),
        type: z.literal("public-key"),
      })
      .array(),
    rp: z.object({
      id: z.string().optional(),
      name: z.string(),
    }),
    timeout: z.number().optional(),
    user: z.object({
      displayName: z.string(),
      id: z.string().transform(base64url.decodeBin),
      name: z.string(),
    }),
  }),
});

export const GetOptions = z.object({
  publicKey: z.object({
    allowCredentials: z
      .object({
        id: z.string().transform(base64url.decodeBin),
        transports: z
          .enum(["ble", "hybrid", "internal", "nfc", "usb"])
          .array()
          .optional(),
        type: z.literal("public-key"),
      })
      .array(),
    attestation: z
      .enum(["none", "direct", "enterprise", "indirect"])
      .optional(),
    attestationFormats: z.string().array().optional(),
    challenge: z.string().transform(base64url.decodeBin),
    extensions: z.record(z.unknown()).optional(),
    rpId: z.string().optional(),
    timeout: z.number().optional(),
    userVerification: z
      .enum(["required", "preferred", "discouraged"])
      .optional(),
    hints: z
      .enum(["security-key", "client-device", "hybrid"])
      .array()
      .optional(),
  }),
});
