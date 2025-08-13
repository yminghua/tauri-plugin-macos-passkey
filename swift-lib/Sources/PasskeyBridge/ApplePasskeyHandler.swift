import Foundation
import AuthenticationServices
import AppKit
import OSLog

public final class ApplePasskeyHandler: NSObject {
    private var registrationContinuation: CheckedContinuation<ASAuthorizationPlatformPublicKeyCredentialRegistration, Error>?
    private var loginContinuation: CheckedContinuation<ASAuthorizationPlatformPublicKeyCredentialAssertion, Error>?

    private let providedWindow: NSWindow?
    
    private let logger = Logger(subsystem: "com.yminghua.tauri-plugin-macos-passkey", category: "ApplePasskeyHandler")

    public init(windowPtr: UnsafeMutableRawPointer?) {
        if let ptr = windowPtr {
            self.providedWindow = Unmanaged<NSWindow>.fromOpaque(ptr).takeUnretainedValue()
            logger.log("Provided window pointer received and converted to NSWindow.")
        } else {
            self.providedWindow = nil
            logger.log("No window pointer provided; fallback to first app window.")
        }
    }

    @available(macOS 15.0, *)
    public func beginRegistration(domain: String, challenge: Data, username: String, userID: Data, salt: Data? = nil) async throws -> ASAuthorizationPlatformPublicKeyCredentialRegistration {
        logger.log("Starting passkey registration for domain: \(domain, privacy: .public), username: \(username, privacy: .public)")
        
        let provider = ASAuthorizationPlatformPublicKeyCredentialProvider(relyingPartyIdentifier: domain)
        let request = provider.createCredentialRegistrationRequest(
            challenge: challenge,
            name: username,
            userID: userID
        )

        // Only add PRF extension if salt is provided
        if let salt = salt {
            let prfInputValues = ASAuthorizationPublicKeyCredentialPRFRegistrationInput.InputValues(saltInput1: salt)
            let prfInput = ASAuthorizationPublicKeyCredentialPRFRegistrationInput.inputValues(prfInputValues)
            request.prf = prfInput
            logger.log("PRF extension enabled for registration")
        } else {
            logger.log("No salt provided - skipping PRF extension for registration")
        }

        let controller = ASAuthorizationController(authorizationRequests: [request])
        controller.delegate = self
        controller.presentationContextProvider = self

        logger.log("Performing registration request...")
        return try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<ASAuthorizationPlatformPublicKeyCredentialRegistration, Error>) in
            self.registrationContinuation = continuation
            controller.performRequests()
        }
    }

    @available(macOS 15.0, *)
    public func beginLogin(domain: String, challenge: Data, salt: Data? = nil) async throws -> ASAuthorizationPlatformPublicKeyCredentialAssertion {
        logger.log("Starting passkey login for domain: \(domain, privacy: .public)")
        
        let provider = ASAuthorizationPlatformPublicKeyCredentialProvider(relyingPartyIdentifier: domain)
        let request = provider.createCredentialAssertionRequest(challenge: challenge)

        // Only add PRF extension if salt is provided
        if let salt = salt {
            let prfInputValues = ASAuthorizationPublicKeyCredentialPRFAssertionInput.InputValues(saltInput1: salt)
            let prfInput = ASAuthorizationPublicKeyCredentialPRFAssertionInput.inputValues(prfInputValues)
            request.prf = prfInput
            logger.log("PRF extension enabled for login")
        } else {
            logger.log("No salt provided — skipping PRF extension for login")
        }

        let controller = ASAuthorizationController(authorizationRequests: [request])
        controller.delegate = self
        controller.presentationContextProvider = self

        logger.log("Performing login request...")
        return try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<ASAuthorizationPlatformPublicKeyCredentialAssertion, Error>) in
            self.loginContinuation = continuation
            controller.performRequests()
        }
    }
}

extension ApplePasskeyHandler: ASAuthorizationControllerDelegate, ASAuthorizationControllerPresentationContextProviding {
    public func presentationAnchor(for controller: ASAuthorizationController) -> ASPresentationAnchor {
       return providedWindow ?? NSApplication.shared.windows.first!
    }

    public func authorizationController(controller: ASAuthorizationController, didCompleteWithAuthorization auth: ASAuthorization) {
        if let registration = auth.credential as? ASAuthorizationPlatformPublicKeyCredentialRegistration {
            logger.info("Passkey registration completed successfully.")
            self.registrationContinuation?.resume(returning: registration)
            self.registrationContinuation = nil
        } else if let assertion = auth.credential as? ASAuthorizationPlatformPublicKeyCredentialAssertion {
            logger.info("Passkey login assertion received successfully.")
            self.loginContinuation?.resume(returning: assertion)
            self.loginContinuation = nil
        } else {
            logger.error("Unsupported credential type received in authorizationController.")
            let error = NSError(domain: "ApplePasskeyHandler", code: -3, userInfo: [NSLocalizedDescriptionKey: "Unsupported credential type."])
            self.registrationContinuation?.resume(throwing: error)
            self.loginContinuation?.resume(throwing: error)
            self.registrationContinuation = nil
            self.loginContinuation = nil
        }
    }

    public func authorizationController(controller: ASAuthorizationController, didCompleteWithError error: Error) {
        logger.error("Authorization controller failed with error: \(error.localizedDescription, privacy: .public)")
        self.registrationContinuation?.resume(throwing: error)
        self.loginContinuation?.resume(throwing: error)
        self.registrationContinuation = nil
        self.loginContinuation = nil
    }
}
