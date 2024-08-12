use actix_web::http;
use phf::phf_map;
use std::{error::Error, fmt};

pub type ApiErrorType = Box<dyn ApiError + Send + Sync>;

pub trait ApiError: Error {
    fn aws_error_code(&self) -> &str;
    fn description(&self) -> &str;
    fn status_code(&self) -> http::StatusCode;
}

pub struct ApiErrorImpl {
    aws_error_code: &'static str,
    description: &'static str,
    status_code: http::StatusCode,
}

impl ApiError for ApiErrorImpl {
    fn aws_error_code(&self) -> &str {
        &self.aws_error_code
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn status_code(&self) -> http::StatusCode {
        self.status_code
    }
}

impl fmt::Display for ApiErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.aws_error_code, self.description)
    }
}

impl fmt::Debug for ApiErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ApiErrorImpl {{ aws_error_code: {}, description: {}, status_code: {} }}",
            self.aws_error_code, self.description, self.status_code
        )
    }
}

impl Error for ApiErrorImpl {}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApiErrorCode {
    ErrAccessDenied,
    ErrBadDigest,
    ErrBucketAlreadyExists,
    ErrEmptyEntity,
    ErrEntityTooLarge,
    ErrIncompleteBody,
    ErrInternalError,
    ErrInvalidAccessKeyID,
    ErrInvalidBucketName,
    ErrInvalidObjectName,
    ErrInvalidDigest,
    ErrInvalidRange,
    ErrInvalidEncodingType,
    ErrInvalidContinuationToken,
    ErrInvalidMaxKeys,
    ErrInvalidMaxUploads,
    ErrInvalidMaxParts,
    ErrInvalidPartNumberMarker,
    ErrInvalidRequestBody,
    ErrInvalidCopySource,
    ErrInvalidCopySourceStorageClass,
    ErrInvalidCopyDest,
    ErrInvalidCopyRequest,
    ErrInvalidCopyRequestWithSameObject,
    ErrInvalidRenameSourceKey,
    ErrInvalidRenameTarget,
    ErrNotSupportBucketEnabledVersion,
    ErrInvalidPrecondition,
    ErrInvalidPolicyDocument,
    ErrInvalidCorsDocument,
    ErrInvalidVersioning,
    ErrMalformedXML,
    ErrMissingContentLength,
    ErrMissingContentMD5,
    ErrMissingRequestBodyError,
    ErrNoSuchBucket,
    ErrNoSuchBucketPolicy,
    ErrNoSuchKey,
    ErrNoSuchUpload,
    ErrNoSuchVersion,
    ErrNotImplemented,
    ErrPreconditionFailed,
    ErrRequestTimeTooSkewed,
    ErrSignatureDoesNotMatch,
    ErrMethodNotAllowed,
    ErrInvalidPart,
    ErrInvalidPartOrder,
    ErrAuthorizationHeaderMalformed,
    ErrMalformedPOSTRequest,
    ErrSignatureVersionNotSupported,
    ErrBucketNotEmpty,
    ErrBucketAccessForbidden,
    ErrMalformedPolicy,
    ErrMissingFields,
    ErrMissingCredTag,
    ErrCredMalformed,
    ErrInvalidRegion,
    ErrInvalidService,
    ErrInvalidRequestVersion,
    ErrMissingSignTag,
    ErrMissingSignHeadersTag,
    ErrMissingRequiredSignedHeader,
    ErrSignedHeadersNotSorted,
    ErrPolicyAlreadyExpired,
    ErrPolicyViolation,
    ErrMalformedDate,
    ErrMalformedExpires,
    ErrAuthHeaderEmpty,
    ErrExpiredPresignRequest,
    ErrMissingDateHeader,
    ErrInvalidQuerySignatureAlgo,
    ErrInvalidQueryParams,
    ErrBucketAlreadyOwnedByYou,
    ErrInvalidCannedAcl,
    ErrInvalidSseHeader,
    ErrTooManyBuckets,
    ErrInvalidPosition,
    ErrObjectNotAppendable,
    ErrPositionNotEqualToLength,
    ErrMetadataHeader,
    ErrMaintenance,
    // Add new error codes here.

    // SSE-S3 related API errors
    ErrInvalidEncryptionMethod,

    // Server-Side-Encryption (with Customer provided key) related API errors.
    ErrInsecureSSECustomerRequest,
    ErrSSEMultipartEncrypted,
    ErrSSEEncryptedObject,
    ErrInvalidEncryptionParameters,
    ErrInvalidSSECustomerAlgorithm,
    ErrInvalidSSECustomerKey,
    ErrMissingSSECustomerKey,
    ErrMissingSSECustomerKeyMD5,
    ErrSSECustomerKeyMD5Mismatch,
    ErrInvalidSSECustomerParameters,
    ErrIncompatibleEncryptionMethod,
    ErrKMSNotConfigured,
    ErrKMSAuthFailure,

    // S3 extended errors.
    ErrContentSHA256Mismatch,
    // Add new extended error codes here.

    // Add new extended error codes here.
    ContentNotModified, // actually not an error
    ErrInvalidHeader,   // supplementary error for golang http lib
    ErrInvalidStatus,
    ErrNoSuchBucketCors,
    ErrPolicyMissingFields,
    ErrInvalidAcl,
    ErrUnsupportedAcl,
    ErrNonUTF8Encode,
    ErrInvalidBucketLogging,
    ErrInvalidLc,
    ErrNoSuchBucketLc,
    ErrInvalidStorageClass,
    ErrInvalidWebsiteConfiguration,
    ErrMalformedWebsiteConfiguration,
    ErrInvalidWebsiteRedirectProtocol,
    ErrExceededWebsiteRoutingRulesLimit,
    ErrSecondLevelDomainForbidden,
    ErrMissingRoutingRuleInWebsiteRules,
    ErrMissingRedirectInWebsiteRoutingRule,
    ErrMissingRedirectElementInWebsiteRoutingRule,
    ErrDuplicateKeyReplaceTagInWebsiteRoutingRule,
    ErrInvalidHttpRedirectCodeInWebsiteRoutingRule,
    ErrIndexDocumentNotAllowed,
    ErrInvalidIndexDocumentSuffix,
    ErrInvalidErrorDocumentKey,
    ErrMalformedMetadataConfiguration,
    ErrMalformedEncryptionConfiguration,
    ErrMissingRuleInEncryption,
    ErrExceededEncryptionRulesLimit,
    ErrMissingEncryptionByDefaultInEncryptionRule,
    ErrMissingSSEAlgorithmOrKMSMasterKeyIDInEncryptionRule,
    ErrInvalidRestoreInfo,
    ErrCreateRestoreObject,
    ErrInvalidGlacierObject,
}

impl ApiErrorCode {
    fn as_str(&self) -> &'static str {
        match self {
            ApiErrorCode::ErrAccessDenied => "ErrAccessDenied",
            ApiErrorCode::ErrBadDigest => "ErrBadDigest",
            ApiErrorCode::ErrBucketAlreadyExists => "ErrBucketAlreadyExists",
            ApiErrorCode::ErrEmptyEntity => "ErrEmptyEntity",
            ApiErrorCode::ErrEntityTooLarge => "ErrEntityTooLarge",
            ApiErrorCode::ErrIncompleteBody => "ErrIncompleteBody",
            ApiErrorCode::ErrInternalError => "ErrInternalError",
            ApiErrorCode::ErrInvalidAccessKeyID => "ErrInvalidAccessKeyID",
            ApiErrorCode::ErrInvalidBucketName => "ErrInvalidBucketName",
            ApiErrorCode::ErrInvalidObjectName => "ErrInvalidObjectName",
            ApiErrorCode::ErrInvalidDigest => "ErrInvalidDigest",
            ApiErrorCode::ErrInvalidRange => "ErrInvalidRange",
            ApiErrorCode::ErrInvalidEncodingType => "ErrInvalidEncodingType",
            ApiErrorCode::ErrInvalidContinuationToken => "ErrInvalidContinuationToken",
            ApiErrorCode::ErrInvalidMaxKeys => "ErrInvalidMaxKeys",
            ApiErrorCode::ErrInvalidMaxUploads => "ErrInvalidMaxUploads",
            ApiErrorCode::ErrInvalidMaxParts => "ErrInvalidMaxParts",
            ApiErrorCode::ErrInvalidPartNumberMarker => "ErrInvalidPartNumberMarker",
            ApiErrorCode::ErrInvalidRequestBody => "ErrInvalidRequestBody",
            ApiErrorCode::ErrInvalidCopySource => "ErrInvalidCopySource",
            ApiErrorCode::ErrInvalidCopySourceStorageClass => "ErrInvalidCopySourceStorageClass",
            ApiErrorCode::ErrInvalidCopyDest => "ErrInvalidCopyDest",
            ApiErrorCode::ErrInvalidCopyRequest => "ErrInvalidCopyRequest",
            ApiErrorCode::ErrInvalidCopyRequestWithSameObject => {
                "ErrInvalidCopyRequestWithSameObject"
            }
            ApiErrorCode::ErrInvalidRenameSourceKey => "ErrInvalidRenameSourceKey",
            ApiErrorCode::ErrInvalidRenameTarget => "ErrInvalidRenameTarget",
            ApiErrorCode::ErrNotSupportBucketEnabledVersion => "ErrNotSupportBucketEnabledVersion",
            ApiErrorCode::ErrInvalidPrecondition => "ErrInvalidPrecondition",
            ApiErrorCode::ErrInvalidPolicyDocument => "ErrInvalidPolicyDocument",
            ApiErrorCode::ErrInvalidCorsDocument => "ErrInvalidCorsDocument",
            ApiErrorCode::ErrInvalidVersioning => "ErrInvalidVersioning",
            ApiErrorCode::ErrMalformedXML => "ErrMalformedXML",
            ApiErrorCode::ErrMissingContentLength => "ErrMissingContentLength",
            ApiErrorCode::ErrMissingContentMD5 => "ErrMissingContentMD5",
            ApiErrorCode::ErrMissingRequestBodyError => "ErrMissingRequestBodyError",
            ApiErrorCode::ErrNoSuchBucket => "ErrNoSuchBucket",
            ApiErrorCode::ErrNoSuchBucketPolicy => "ErrNoSuchBucketPolicy",
            ApiErrorCode::ErrNoSuchKey => "ErrNoSuchKey",
            ApiErrorCode::ErrNoSuchUpload => "ErrNoSuchUpload",
            ApiErrorCode::ErrNoSuchVersion => "ErrNoSuchVersion",
            ApiErrorCode::ErrNotImplemented => "ErrNotImplemented",
            ApiErrorCode::ErrPreconditionFailed => "ErrPreconditionFailed",
            ApiErrorCode::ErrRequestTimeTooSkewed => "ErrRequestTimeTooSkewed",
            ApiErrorCode::ErrSignatureDoesNotMatch => "ErrSignatureDoesNotMatch",
            ApiErrorCode::ErrMethodNotAllowed => "ErrMethodNotAllowed",
            ApiErrorCode::ErrInvalidPart => "ErrInvalidPart",
            ApiErrorCode::ErrInvalidPartOrder => "ErrInvalidPartOrder",
            ApiErrorCode::ErrAuthorizationHeaderMalformed => "ErrAuthorizationHeaderMalformed",
            ApiErrorCode::ErrMalformedPOSTRequest => "ErrMalformedPOSTRequest",
            ApiErrorCode::ErrSignatureVersionNotSupported => "ErrSignatureVersionNotSupported",
            ApiErrorCode::ErrBucketNotEmpty => "ErrBucketNotEmpty",
            ApiErrorCode::ErrBucketAccessForbidden => "ErrBucketAccessForbidden",
            ApiErrorCode::ErrMalformedPolicy => "ErrMalformedPolicy",
            ApiErrorCode::ErrMissingFields => "ErrMissingFields",
            ApiErrorCode::ErrMissingCredTag => "ErrMissingCredTag",
            ApiErrorCode::ErrCredMalformed => "ErrCredMalformed",
            ApiErrorCode::ErrInvalidRegion => "ErrInvalidRegion",
            ApiErrorCode::ErrInvalidService => "ErrInvalidService",
            ApiErrorCode::ErrInvalidRequestVersion => "ErrInvalidRequestVersion",
            ApiErrorCode::ErrMissingSignTag => "ErrMissingSignTag",
            ApiErrorCode::ErrMissingSignHeadersTag => "ErrMissingSignHeadersTag",
            ApiErrorCode::ErrMissingRequiredSignedHeader => "ErrMissingRequiredSignedHeader",
            ApiErrorCode::ErrSignedHeadersNotSorted => "ErrSignedHeadersNotSorted",
            ApiErrorCode::ErrPolicyAlreadyExpired => "ErrPolicyAlreadyExpired",
            ApiErrorCode::ErrPolicyViolation => "ErrPolicyViolation",
            ApiErrorCode::ErrMalformedDate => "ErrMalformedDate",
            ApiErrorCode::ErrMalformedExpires => "ErrMalformedExpires",
            ApiErrorCode::ErrAuthHeaderEmpty => "ErrAuthHeaderEmpty",
            ApiErrorCode::ErrExpiredPresignRequest => "ErrExpiredPresignRequest",
            ApiErrorCode::ErrMissingDateHeader => "ErrMissingDateHeader",
            ApiErrorCode::ErrInvalidQuerySignatureAlgo => "ErrInvalidQuerySignatureAlgo",
            ApiErrorCode::ErrInvalidQueryParams => "ErrInvalidQueryParams",
            ApiErrorCode::ErrBucketAlreadyOwnedByYou => "ErrBucketAlreadyOwnedByYou",
            ApiErrorCode::ErrInvalidCannedAcl => "ErrInvalidCannedAcl",
            ApiErrorCode::ErrInvalidSseHeader => "ErrInvalidSseHeader",
            ApiErrorCode::ErrTooManyBuckets => "ErrTooManyBuckets",
            ApiErrorCode::ErrInvalidPosition => "ErrInvalidPosition",
            ApiErrorCode::ErrObjectNotAppendable => "ErrObjectNotAppendable",
            ApiErrorCode::ErrPositionNotEqualToLength => "ErrPositionNotEqualToLength",
            ApiErrorCode::ErrMetadataHeader => "ErrMetadataHeader",
            ApiErrorCode::ErrMaintenance => "ErrMaintenance",
            ApiErrorCode::ErrInvalidEncryptionMethod => "ErrInvalidEncryptionMethod",
            ApiErrorCode::ErrInsecureSSECustomerRequest => "ErrInsecureSSECustomerRequest",
            ApiErrorCode::ErrSSEMultipartEncrypted => "ErrSSEMultipartEncrypted",
            ApiErrorCode::ErrSSEEncryptedObject => "ErrSSEEncryptedObject",
            ApiErrorCode::ErrInvalidEncryptionParameters => "ErrInvalidEncryptionParameters",
            ApiErrorCode::ErrInvalidSSECustomerAlgorithm => "ErrInvalidSSECustomerAlgorithm",
            ApiErrorCode::ErrInvalidSSECustomerKey => "ErrInvalidSSECustomerKey",
            ApiErrorCode::ErrMissingSSECustomerKey => "ErrMissingSSECustomerKey",
            ApiErrorCode::ErrMissingSSECustomerKeyMD5 => "ErrMissingSSECustomerKeyMD5",
            ApiErrorCode::ErrSSECustomerKeyMD5Mismatch => "ErrSSECustomerKeyMD5Mismatch",
            ApiErrorCode::ErrInvalidSSECustomerParameters => "ErrInvalidSSECustomerParameters",
            ApiErrorCode::ErrIncompatibleEncryptionMethod => "ErrIncompatibleEncryptionMethod",
            ApiErrorCode::ErrKMSNotConfigured => "ErrKMSNotConfigured",
            ApiErrorCode::ErrKMSAuthFailure => "ErrKMSAuthFailure",

            ApiErrorCode::ErrContentSHA256Mismatch => "ErrContentSHA256Mismatch",

            ApiErrorCode::ContentNotModified => "ContentNotModified",
            ApiErrorCode::ErrInvalidHeader => "ErrInvalidHeader",
            ApiErrorCode::ErrInvalidStatus => "ErrInvalidStatus",
            ApiErrorCode::ErrNoSuchBucketCors => "ErrNoSuchBucketCors",
            ApiErrorCode::ErrPolicyMissingFields => "ErrPolicyMissingFields",
            ApiErrorCode::ErrInvalidAcl => "ErrInvalidAcl",
            ApiErrorCode::ErrUnsupportedAcl => "ErrUnsupportedAcl",
            ApiErrorCode::ErrNonUTF8Encode => "ErrNonUTF8Encode",
            ApiErrorCode::ErrInvalidBucketLogging => "ErrInvalidBucketLogging",
            ApiErrorCode::ErrInvalidLc => "ErrInvalidLc",
            ApiErrorCode::ErrNoSuchBucketLc => "ErrNoSuchBucketLc",
            ApiErrorCode::ErrInvalidStorageClass => "ErrInvalidStorageClass",
            ApiErrorCode::ErrInvalidWebsiteConfiguration => "ErrInvalidWebsiteConfiguration",
            ApiErrorCode::ErrMalformedWebsiteConfiguration => "ErrMalformedWebsiteConfiguration",
            ApiErrorCode::ErrInvalidWebsiteRedirectProtocol => "ErrInvalidWebsiteRedirectProtocol",
            ApiErrorCode::ErrExceededWebsiteRoutingRulesLimit => {
                "ErrExceededWebsiteRoutingRulesLimit"
            }
            ApiErrorCode::ErrSecondLevelDomainForbidden => "ErrSecondLevelDomainForbidden",
            ApiErrorCode::ErrMissingRoutingRuleInWebsiteRules => {
                "ErrMissingRoutingRuleInWebsiteRules"
            }
            ApiErrorCode::ErrMissingRedirectInWebsiteRoutingRule => {
                "ErrMissingRedirectInWebsiteRoutingRule"
            }
            ApiErrorCode::ErrMissingRedirectElementInWebsiteRoutingRule => {
                "ErrMissingRedirectElementInWebsiteRoutingRule"
            }
            ApiErrorCode::ErrDuplicateKeyReplaceTagInWebsiteRoutingRule => {
                "ErrDuplicateKeyReplaceTagInWebsiteRoutingRule"
            }
            ApiErrorCode::ErrInvalidHttpRedirectCodeInWebsiteRoutingRule => {
                "ErrInvalidHttpRedirectCodeInWebsiteRoutingRule"
            }
            ApiErrorCode::ErrIndexDocumentNotAllowed => "ErrIndexDocumentNotAllowed",
            ApiErrorCode::ErrInvalidIndexDocumentSuffix => "ErrInvalidIndexDocumentSuffix",
            ApiErrorCode::ErrInvalidErrorDocumentKey => "ErrInvalidErrorDocumentKey",
            ApiErrorCode::ErrMalformedMetadataConfiguration => "ErrMalformedMetadataConfiguration",
            ApiErrorCode::ErrMalformedEncryptionConfiguration => {
                "ErrMalformedEncryptionConfiguration"
            }
            ApiErrorCode::ErrMissingRuleInEncryption => "ErrMissingRuleInEncryption",
            ApiErrorCode::ErrExceededEncryptionRulesLimit => "ErrExceededEncryptionRulesLimit",
            ApiErrorCode::ErrMissingEncryptionByDefaultInEncryptionRule => {
                "ErrMissingEncryptionByDefaultInEncryptionRule"
            }
            ApiErrorCode::ErrMissingSSEAlgorithmOrKMSMasterKeyIDInEncryptionRule => {
                "ErrMissingSSEAlgorithmOrKMSMasterKeyIDInEncryptionRule"
            }
            ApiErrorCode::ErrInvalidRestoreInfo => "ErrInvalidRestoreInfo",
            ApiErrorCode::ErrCreateRestoreObject => "ErrCreateRestoreObject",
            ApiErrorCode::ErrInvalidGlacierObject => "ErrInvalidGlacierObject",
        }
    }
}

const INTERNAL_SERVER_ERROR: ApiErrorImpl = ApiErrorImpl {
  aws_error_code: "InternalError",
  description: "We encountered an internal error, please try again.",
  status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
};

static ERROR_CODE_REPONSE: phf::Map<&'static str, ApiErrorImpl> = phf_map! {
  "ErrInvalidCopyDest" => ApiErrorImpl { aws_error_code: "InvalidRequest", description: "This copy request is illegal because it is trying to copy an object to itself", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidCopySource" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "Copy Source must mention the source bucket and key: sourcebucket/sourcekey", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidRestoreInfo" => ApiErrorImpl { aws_error_code: "InvalidRestoreInfo", description: "Defrost parameter setting error.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidCopySourceStorageClass" => ApiErrorImpl { aws_error_code: "InvalidCopySourceStorageClass", description: "Storage class of copy source cannot be GLACIER or DEEP_ARCHIVE", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidCopyRequest" => ApiErrorImpl { aws_error_code: "InvalidCopyRequest", description: "X-Amz-Metadata-Directive should be COPY or REPLACE", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidCopyRequestWithSameObject" => ApiErrorImpl { aws_error_code: "InvalidCopyRequest", description: "This copy request is illegal because it is trying to copy an object to itself without changing the object's metadata, storage class, website redirect location or encryption attributes.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidRenameSourceKey" => ApiErrorImpl { aws_error_code: "InvalidRenameSourceKey", description: "X-Amz-Rename-Source-Key must be a valid URL-encoded object name, renaming folders is not supported.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidRenameTarget" => ApiErrorImpl { aws_error_code: "InvalidRenameTarget", description: "Rename Target must not be a folder and addition target have not already created.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrNotSupportBucketEnabledVersion" => ApiErrorImpl { aws_error_code: "NotSupported", description: "Renaming objects in multi-version enabled buckets is not supported.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidPrecondition" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The provided preconditions are not valid(bad time format, rule combination, etc)", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidRequestBody" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "Body shouldn't be set for this request", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidEncodingType" => ApiErrorImpl { aws_error_code: "InvalidEncodingType", description: "The encoding type specified is not allowed.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidContinuationToken" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The continuation token provided is invalid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidMaxUploads" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "Argument max-uploads must be an integer between 1 and 1000", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidMaxKeys" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "Argument max-keys must be an integer between 1 and 1000", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidMaxParts" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "Argument max-parts must be an integer between 1 and 1000", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidPartNumberMarker" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "Argument part-number-marker must be an integer between 0 and 10000", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidPolicyDocument" => ApiErrorImpl { aws_error_code: "MalformedPolicy", description: "Policy has invalid resource", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidCorsDocument" => ApiErrorImpl { aws_error_code: "MalformedPolicy", description: "CORS has invalid resource", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidVersioning" => ApiErrorImpl { aws_error_code: "IllegalVersioningConfigurationException", description: "The versioning configuration is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidGlacierObject" => ApiErrorImpl { aws_error_code: "InvalidGlacierObject", description: "The operation is not valid for the object's storage class.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrAccessDenied" => ApiErrorImpl { aws_error_code: "AccessDenied", description: "Access Denied.", status_code: http::StatusCode::FORBIDDEN },
  "ErrBadDigest" => ApiErrorImpl { aws_error_code: "BadDigest", description: "The Content-MD5 you specified did not match what we received.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrBucketAlreadyExists" => ApiErrorImpl { aws_error_code: "BucketAlreadyExists", description: "The requested bucket name is not available.", status_code: http::StatusCode::CONFLICT },
  "ErrEmptyEntity" => ApiErrorImpl { aws_error_code: "EmptyEntity", description: "Request body is empty.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrEntityTooLarge" => ApiErrorImpl { aws_error_code: "EntityTooLarge", description: "Your proposed upload exceeds the maximum allowed object size.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrIncompleteBody" => ApiErrorImpl { aws_error_code: "IncompleteBody", description: "You did not provide the number of bytes specified by the Content-Length HTTP header.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInternalError" => INTERNAL_SERVER_ERROR,
  "ErrInvalidAccessKeyID" => ApiErrorImpl { aws_error_code: "InvalidAccessKeyId", description: "The AWS Access Key Id you provided does not exist in our records.", status_code: http::StatusCode::FORBIDDEN },
  "ErrInvalidBucketName" => ApiErrorImpl { aws_error_code: "InvalidBucketName", description: "The specified bucket name is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidObjectName" => ApiErrorImpl { aws_error_code: "InvalidObjectName", description: "The specified object name is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidDigest" => ApiErrorImpl { aws_error_code: "InvalidDigest", description: "The Content-MD5 you specified is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidRange" => ApiErrorImpl { aws_error_code: "InvalidRange", description: "The requested range is not satisfiable.", status_code: http::StatusCode::RANGE_NOT_SATISFIABLE },
  "ErrMalformedXML" => ApiErrorImpl { aws_error_code: "MalformedXML", description: "The XML you provided was not well-formed or did not validate against our published schema.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingContentLength" => ApiErrorImpl { aws_error_code: "MissingContentLength", description: "You must provide the Content-Length HTTP header.", status_code: http::StatusCode::LENGTH_REQUIRED },
  "ErrMissingContentMD5" => ApiErrorImpl { aws_error_code: "MissingContentMD5", description: "Missing required header for this request: Content-MD5", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingRequestBodyError" => ApiErrorImpl { aws_error_code: "MissingRequestBodyError", description: "Request body is empty.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrNoSuchBucket" => ApiErrorImpl { aws_error_code: "NoSuchBucket", description: "The specified bucket does not exist.", status_code: http::StatusCode::NOT_FOUND },
  "ErrNoSuchBucketPolicy" => ApiErrorImpl { aws_error_code: "NoSuchBucketPolicy", description: "The bucket policy does not exist.", status_code: http::StatusCode::NOT_FOUND },
  "ErrNoSuchKey" => ApiErrorImpl { aws_error_code: "NoSuchKey", description: "The specified key does not exist.", status_code: http::StatusCode::NOT_FOUND },
  "ErrNoSuchUpload" => ApiErrorImpl { aws_error_code: "NoSuchUpload", description: "The specified multipart upload does not exist.", status_code: http::StatusCode::NOT_FOUND },
  "ErrNoSuchVersion" => ApiErrorImpl { aws_error_code: "NoSuchVersion", description: "The specified version does not exist.", status_code: http::StatusCode::NOT_FOUND },
  "ErrNotImplemented" => ApiErrorImpl { aws_error_code: "NotImplemented", description: "A header you provided implies functionality that is not implemented.", status_code: http::StatusCode::NOT_IMPLEMENTED },
  "ErrPreconditionFailed" => ApiErrorImpl { aws_error_code: "PreconditionFailed", description: "At least one of the preconditions you specified did not hold.", status_code: http::StatusCode::PRECONDITION_FAILED },
  "ErrRequestTimeTooSkewed" => ApiErrorImpl { aws_error_code: "RequestTimeTooSkewed", description: "The difference between the request time and the server's time is too large.", status_code: http::StatusCode::FORBIDDEN },
  "ErrSignatureDoesNotMatch" => ApiErrorImpl { aws_error_code: "SignatureDoesNotMatch", description: "The request signature we calculated does not match the signature you provided.", status_code: http::StatusCode::FORBIDDEN },
  "ErrMethodNotAllowed" => ApiErrorImpl { aws_error_code: "MethodNotAllowed", description: "The specified method is not allowed against this resource.", status_code: http::StatusCode::METHOD_NOT_ALLOWED },
  "ErrInvalidPart" => ApiErrorImpl { aws_error_code: "InvalidPart", description: "One or more of the specified parts could not be found.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidPartOrder" => ApiErrorImpl { aws_error_code: "InvalidPartOrder", description: "The list of parts was not in ascending order.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrAuthorizationHeaderMalformed" => ApiErrorImpl { aws_error_code: "AuthorizationHeaderMalformed", description: "The authorization header is malformed.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMalformedPOSTRequest" => ApiErrorImpl { aws_error_code: "MalformedPOSTRequest", description: "The body of your POST request is not well-formed multipart/form-data.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrSignatureVersionNotSupported" => ApiErrorImpl { aws_error_code: "SignatureVersionNotSupported", description: "The requested signature version is not supported.", status_code: http::StatusCode::FORBIDDEN },
  "ErrBucketNotEmpty" => ApiErrorImpl { aws_error_code: "BucketNotEmpty", description: "The bucket you tried to delete is not empty.", status_code: http::StatusCode::CONFLICT },
  "ErrBucketAccessForbidden" => ApiErrorImpl { aws_error_code: "AccessForbidden", description: "Access to the bucket is forbidden.", status_code: http::StatusCode::FORBIDDEN },
  "ErrMalformedPolicy" => ApiErrorImpl { aws_error_code: "MalformedPolicy", description: "The policy provided is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingFields" => ApiErrorImpl { aws_error_code: "MissingFields", description: "Request is missing required fields.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingCredTag" => ApiErrorImpl { aws_error_code: "MissingCredTag", description: "Request is missing credential tag.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrCredMalformed" => ApiErrorImpl { aws_error_code: "CredMalformed", description: "Request credential is malformed.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMalformedDate" => ApiErrorImpl { aws_error_code: "MalformedDate", description: "Request date is malformed.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidRegion" => ApiErrorImpl { aws_error_code: "InvalidRegion", description: "The specified region is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidService" => ApiErrorImpl { aws_error_code: "AccessDenied", description: "The specified service is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidRequestVersion" => ApiErrorImpl { aws_error_code: "AccessDenied", description: "The specified request version is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingSignTag" => ApiErrorImpl { aws_error_code: "AccessDenied", description: "Request is missing sign tag.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingSignHeadersTag" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "Request is missing sign headers tag.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingRequiredSignedHeader" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "Request is missing required signed header.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrSignedHeadersNotSorted" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "Request signed headers are not sorted.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrPolicyAlreadyExpired" => ApiErrorImpl { aws_error_code: "AccessDenied", description: "Policy has already expired.", status_code: http::StatusCode::FORBIDDEN },
  "ErrPolicyViolation" => ApiErrorImpl { aws_error_code: "AccessDenied", description: "Policy violation.", status_code: http::StatusCode::FORBIDDEN },
  "ErrMalformedExpires" => ApiErrorImpl { aws_error_code: "MalformedExpires", description: "Malformed expires value, should be between 1 and 604800(seven days)", status_code: http::StatusCode::BAD_REQUEST },
  "ErrAuthHeaderEmpty" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "Authorization header is empty.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingDateHeader" => ApiErrorImpl { aws_error_code: "AccessDenied", description: "Date header is missing.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidQuerySignatureAlgo" => ApiErrorImpl { aws_error_code: "AuthorizationQueryParametersError", description: "Query signature algorithm is invalid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrExpiredPresignRequest" => ApiErrorImpl { aws_error_code: "ExpiredToken", description: "Presign request has expired.", status_code: http::StatusCode::FORBIDDEN },
  "ErrInvalidQueryParams" => ApiErrorImpl { aws_error_code: "AuthorizationQueryParametersError", description: "Query parameters are invalid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrBucketAlreadyOwnedByYou" => ApiErrorImpl { aws_error_code: "BucketAlreadyOwnedByYou", description: "Your previous request to create the named bucket succeeded and you already own it.", status_code: http::StatusCode::CONFLICT },
  "ErrTooManyBuckets" => ApiErrorImpl { aws_error_code: "TooManyBuckets", description: "You have attempted to create more buckets than allowed.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidEncryptionMethod" => ApiErrorImpl { aws_error_code: "InvalidRequest", description: "The encryption method specified is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInsecureSSECustomerRequest" => ApiErrorImpl { aws_error_code: "InvalidRequest", description: "Requests specifying Server Side Encryption with Customer provided keys should use HTTPS.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrSSEMultipartEncrypted" => ApiErrorImpl { aws_error_code: "InvalidRequest", description: "The multipart upload initiation request specified server side encryption with customer provided keys but no encryption key.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrSSEEncryptedObject" => ApiErrorImpl { aws_error_code: "InvalidRequest", description: "The object was stored using a form of Server Side Encryption with Customer provided keys but was not being requested to be downloaded using customer provided keys.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidEncryptionParameters" => ApiErrorImpl { aws_error_code: "InvalidRequest", description: "The encryption parameters specified are not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidSSECustomerAlgorithm" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The SSE-C algorithm specified is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidSSECustomerKey" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The SSE-C key provided is invalid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingSSECustomerKey" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The SSE-C key is required for this operation.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingSSECustomerKeyMD5" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The SSE-C key MD5 is required for this operation.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrSSECustomerKeyMD5Mismatch" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The provided SSE-C key and MD5 do not match.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidSSECustomerParameters" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The SSE-C parameters are not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrIncompatibleEncryptionMethod" => ApiErrorImpl { aws_error_code: "InvalidRequest", description: "The encryption method specified is not compatible with the specified storage class.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrKMSNotConfigured" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The KMS key is not configured properly.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrKMSAuthFailure" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The KMS key authorization failed.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrContentSHA256Mismatch" => ApiErrorImpl { aws_error_code: "XAmzContentSHA256Mismatch", description: "The provided 'x-amz-content-sha256' header does not match what was computed.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidCanndAcl" => ApiErrorImpl { aws_error_code: "InvalidAcl", description: "The canned ACL specified is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidSseHeader" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The SSE header is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ContentNotModified" => ApiErrorImpl { aws_error_code: "NotModified", description: "The requested resource has not been modified since the specified time.", status_code: http::StatusCode::NOT_MODIFIED },
  "ErrInvalidHeader" => ApiErrorImpl { aws_error_code: "InvalidStatus", description: "The header provided is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidStatus" => ApiErrorImpl { aws_error_code: "InvalidStatus", description: "The status provided is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrNoSuchBucketCors" => ApiErrorImpl { aws_error_code: "NoSuchCORSConfiguration", description: "The CORS configuration does not exist.", status_code: http::StatusCode::NOT_FOUND },
  "ErrPolicyMissingFields" => ApiErrorImpl { aws_error_code: "AccessDenied", description: "Policy is missing required fields.", status_code: http::StatusCode::FORBIDDEN },
  "ErrInvalidAcl" => ApiErrorImpl { aws_error_code: "IllegalAclConfigurationException", description: "The ACL provided is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrUnsupportedAcl" => ApiErrorImpl { aws_error_code: "UnsupportedAclConfigurationException", description: "The ACL provided is not supported.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrNonUTF8Encode" => ApiErrorImpl { aws_error_code: "InvalidArgument", description: "The object name is not valid UTF-8 encoded.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrNoSuchBucketLc" => ApiErrorImpl { aws_error_code: "NoSuchLifecycleConfiguration", description: "The lifecycle configuration does not exist.", status_code: http::StatusCode::NOT_FOUND },
  "ErrInvalidLc" => ApiErrorImpl { aws_error_code: "MalformedLifecycleConfiguration", description: "The lifecycle configuration provided is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidPosition" => ApiErrorImpl { aws_error_code: "InvalidPosition", description: "The position specified is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrObjectNotAppendable" => ApiErrorImpl { aws_error_code: "InvalidObjectState", description: "The object is not appendable.", status_code: http::StatusCode::CONFLICT },
  "ErrPositionNotEqualToLength" => ApiErrorImpl { aws_error_code: "InvalidPosition", description: "The position specified is not equal to the object length.", status_code: http::StatusCode::CONFLICT },
  "ErrInvalidStorageClass" => ApiErrorImpl { aws_error_code: "InvalidStorageClass", description: "The storage class specified is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidWebsiteConfiguration" => ApiErrorImpl { aws_error_code: "MalformedXML", description: "The website configuration provided is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMalformedWebsiteConfiguration" => ApiErrorImpl { aws_error_code: "MalformedXML", description: "The website configuration provided is not well-formed.", status_code: http::StatusCode::CONFLICT },
  "ErrInvalidWebsiteRedirectProtocol" => ApiErrorImpl { aws_error_code: "InvalidRedirectLocation", description: "The website redirect location protocol is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrExceededWebsiteRoutingRulesLimit" => ApiErrorImpl { aws_error_code: "TooManyWebsiteRules", description: "The website routing rules limit is exceeded.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrSecondLevelDomainForbidden" => ApiErrorImpl { aws_error_code: "InvalidDomainName", description: "The second level domain is forbidden.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingRoutingRuleInWebsiteRules" => ApiErrorImpl { aws_error_code: "MissingRoutingRule", description: "The routing rule is missing in the website rules.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingRedirectInWebsiteRoutingRule" => ApiErrorImpl { aws_error_code: "MissingRedirectLocation", description: "The redirect location is missing in the website routing rule.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingRedirectElementInWebsiteRoutingRule" => ApiErrorImpl { aws_error_code: "MissingRedirectElement", description: "The redirect element is missing in the website routing rule.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrDuplicateKeyReplaceTagInWebsiteRoutingRule" => ApiErrorImpl { aws_error_code: "DuplicateKeyReplaceTag", description: "The key replace tag is duplicated in the website routing rule.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidHttpRedirectCodeInWebsiteRoutingRule" => ApiErrorImpl { aws_error_code: "InvalidHttpRedirectCode", description: "The HTTP redirect code is not valid in the website routing rule.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrIndexDocumentNotAllowed" => ApiErrorImpl { aws_error_code: "IndexDocumentNotAllowed", description: "The index document is not allowed.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidIndexDocumentSuffix" => ApiErrorImpl { aws_error_code: "InvalidIndexDocumentSuffix", description: "The index document suffix is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrInvalidErrorDocumentKey" => ApiErrorImpl { aws_error_code: "InvalidErrorDocumentKey", description: "The error document key is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMetadataHeader" => ApiErrorImpl { aws_error_code: "InvalidMetadata", description: "The metadata header is not valid.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMalformedMetadataConfiguration" => ApiErrorImpl { aws_error_code: "MalformedXML", description: "The metadata configuration provided is not well-formed.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMalformedEncryptionConfiguration" => ApiErrorImpl { aws_error_code: "MalformedXML", description: "The encryption configuration provided is not well-formed.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingRuleInEncryption" => ApiErrorImpl { aws_error_code: "MissingRuleInEncryption", description: "The rule is missing in the encryption configuration.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingEncryptionByDefaultInEncryptionRule" => ApiErrorImpl { aws_error_code: "MissingEncryptionByDefault", description: "The encryption by default is missing in the encryption rule.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrMissingSSEAlgorithmOrKMSMasterKeyIDInEncryptionRule" => ApiErrorImpl { aws_error_code: "MissingSSEAlgorithmOrKMSMasterKeyID", description: "The SSE algorithm or KMS master key ID is missing in the encryption rule.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrExceededEncryptionRulesLimit" => ApiErrorImpl { aws_error_code: "TooManyEncryptionRules", description: "The encryption rules limit is exceeded.", status_code: http::StatusCode::BAD_REQUEST },
  "ErrCreateRestoreObject" => ApiErrorImpl { aws_error_code: "InvalidRestoreObject", description: "The operation is not valid for the object's storage class.", status_code: http::StatusCode::INTERNAL_SERVER_ERROR },
  "ErrMaintenance" => ApiErrorImpl { aws_error_code: "Maintenance", description: "The server is under maintenance, please try again later.", status_code: http::StatusCode::SERVICE_UNAVAILABLE },
};

impl ApiErrorCode {
    pub fn aws_error_code(&self) -> &'static str {
        ERROR_CODE_REPONSE
            .get(&self.as_str())
            .unwrap_or(&INTERNAL_SERVER_ERROR)
            .aws_error_code
    }

    pub fn description(&self) -> &'static str {
        ERROR_CODE_REPONSE
            .get(&self.as_str())
            .unwrap_or(&INTERNAL_SERVER_ERROR)
            .description
    }

    pub fn status_code(&self) -> http::StatusCode {
        ERROR_CODE_REPONSE
            .get(&self.as_str())
            .unwrap_or(&INTERNAL_SERVER_ERROR)
            .status_code
    }
}
