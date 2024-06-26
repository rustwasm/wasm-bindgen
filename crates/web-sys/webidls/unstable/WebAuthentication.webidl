partial interface PublicKeyCredential {
    [SameObject] readonly attribute DOMString?               authenticatorAttachment;
    static Promise<boolean> isConditionalMediationAvailable();
    [Throws] PublicKeyCredentialJSON toJSON();
};

typedef DOMString Base64URLString;
typedef (RegistrationResponseJSON or AuthenticationResponseJSON) PublicKeyCredentialJSON;

dictionary RegistrationResponseJSON {
    required Base64URLString id;
    required Base64URLString rawId;
    required AuthenticatorAttestationResponseJSON response;
    DOMString authenticatorAttachment;
    required AuthenticationExtensionsClientOutputsJSON clientExtensionResults;
    required DOMString type;
};

dictionary AuthenticatorAttestationResponseJSON {
    required Base64URLString clientDataJSON;
    required Base64URLString authenticatorData;
    required sequence<DOMString> transports;
    // The publicKey field will be missing if pubKeyCredParams was used to
    // negotiate a public-key algorithm that the user agent doesn’t
    // understand. (See section “Easily accessing credential data” for a
    // list of which algorithms user agents must support.) If using such an
    // algorithm then the public key must be parsed directly from
    // attestationObject or authenticatorData.
    Base64URLString publicKey;
    required long long publicKeyAlgorithm;
    // This value contains copies of some of the fields above. See
    // section “Easily accessing credential data”.
    required Base64URLString attestationObject;
};

dictionary AuthenticationResponseJSON {
    required Base64URLString id;
    required Base64URLString rawId;
    required AuthenticatorAssertionResponseJSON response;
    DOMString authenticatorAttachment;
    required AuthenticationExtensionsClientOutputsJSON clientExtensionResults;
    required DOMString type;
};

dictionary AuthenticatorAssertionResponseJSON {
    required Base64URLString clientDataJSON;
    required Base64URLString authenticatorData;
    required Base64URLString signature;
    Base64URLString userHandle;
    Base64URLString attestationObject;
};

dictionary AuthenticationExtensionsClientOutputsJSON {
};

partial interface PublicKeyCredential {
    static Promise<boolean> isPasskeyPlatformAuthenticatorAvailable();
};

partial interface PublicKeyCredential {
    [Throws] static PublicKeyCredentialCreationOptions parseCreationOptionsFromJSON(PublicKeyCredentialCreationOptionsJSON options);
};

dictionary PublicKeyCredentialCreationOptionsJSON {
    required PublicKeyCredentialRpEntity                    rp;
    required PublicKeyCredentialUserEntityJSON              user;
    required Base64URLString                                challenge;
    required sequence<PublicKeyCredentialParameters>        pubKeyCredParams;
    unsigned long                                           timeout;
    sequence<PublicKeyCredentialDescriptorJSON>             excludeCredentials = [];
    AuthenticatorSelectionCriteria                          authenticatorSelection;
    sequence<DOMString>                                     hints = [];
    DOMString                                               attestation = "none";
    sequence<DOMString>                                     attestationFormats = [];
    AuthenticationExtensionsClientInputsJSON                extensions;
};

dictionary PublicKeyCredentialUserEntityJSON {
    required Base64URLString        id;
    required DOMString              name;
    required DOMString              displayName;
};

dictionary PublicKeyCredentialDescriptorJSON {
    required Base64URLString        id;
    required DOMString              type;
    sequence<DOMString>             transports;
};

dictionary AuthenticationExtensionsClientInputsJSON {
};

partial interface PublicKeyCredential {
    [Throws] static PublicKeyCredentialRequestOptions parseRequestOptionsFromJSON(PublicKeyCredentialRequestOptionsJSON options);
};

dictionary PublicKeyCredentialRequestOptionsJSON {
    required Base64URLString                                challenge;
    unsigned long                                           timeout;
    DOMString                                               rpId;
    sequence<PublicKeyCredentialDescriptorJSON>             allowCredentials = [];
    DOMString                                               userVerification = "preferred";
    sequence<DOMString>                                     hints = [];
    DOMString                                               attestation = "none";
    sequence<DOMString>                                     attestationFormats = [];
    AuthenticationExtensionsClientInputsJSON                extensions;
};

partial interface AuthenticatorAssertionResponse {
    [SameObject] readonly attribute ArrayBuffer?     attestationObject;
};

partial dictionary PublicKeyCredentialCreationOptions {
    sequence<DOMString>                          hints = [];
    sequence<DOMString>                          attestationFormats = [];
};

partial dictionary PublicKeyCredentialRequestOptions {
    sequence<DOMString>                  hints = [];
    DOMString                            attestation = "none";
    sequence<DOMString>                  attestationFormats = [];
};

/* Deprecated.
partial dictionary CollectedClientData {
    TokenBinding                 tokenBinding;
};
*/

/* TODO: requires partial or extend support.
enum AuthenticatorTransport {
    "smart-card",
    "hybrid",
};
*/

enum PublicKeyCredentialHints {
    "security-key",
    "client-device",
    "hybrid",
};

dictionary AuthenticationExtensionsPRFValues {
    required BufferSource first;
    BufferSource second;
};

dictionary AuthenticationExtensionsPRFInputs {
    AuthenticationExtensionsPRFValues eval;
    record<USVString, AuthenticationExtensionsPRFValues> evalByCredential;
};

partial dictionary AuthenticationExtensionsClientInputs {
    AuthenticationExtensionsPRFInputs prf;
};

dictionary AuthenticationExtensionsPRFOutputs {
    boolean enabled;
    AuthenticationExtensionsPRFValues results;
};

partial dictionary AuthenticationExtensionsClientOutputs {
    AuthenticationExtensionsPRFOutputs prf;
};

dictionary AuthenticationExtensionsDevicePublicKeyInputs {
    DOMString attestation = "none";
    sequence<DOMString> attestationFormats = [];
};

partial dictionary AuthenticationExtensionsClientInputs {
    AuthenticationExtensionsDevicePublicKeyInputs devicePubKey;
};

dictionary AuthenticationExtensionsDevicePublicKeyOutputs {
    ArrayBuffer signature;
};

partial dictionary AuthenticationExtensionsClientOutputs {
    AuthenticationExtensionsDevicePublicKeyOutputs devicePubKey;
};
