#![allow(dead_code)]
#![allow(unused_variables)]

type Bit = bool; // 0 or 1
type Byte = u8; // 8 bits
type TwoByteInteger = u16; // 16 bits
type FourByteInteger = u32; // 32 bits
type VariableByteInteger = u32; // 1 to 4 bytes
type UTF8EncodedString = String; // UTF-8 Encoded String
type BinaryData = Vec<u8>; // Binary Data
type UTF8StringPair = (String, String); // UTF-8 String Pair

// MQTT Control Packet Type
const MQTT_CONTROL_PACKET_TYPE_CONNECT: Byte = 1; // Client request to connect to Server
const MQTT_CONTROL_PACKET_TYPE_CONNACK: Byte = 2; // Connect acknowledgment
const MQTT_CONTROL_PACKET_TYPE_PUBLISH: Byte = 3; // Publish message
const MQTT_CONTROL_PACKET_TYPE_PUBACK: Byte = 4; // Publish acknowledgment
const MQTT_CONTROL_PACKET_TYPE_PUBREC: Byte = 5; // Publish received (assured delivery part 1)
const MQTT_CONTROL_PACKET_TYPE_PUBREL: Byte = 6; // Publish release (assured delivery part 2)
const MQTT_CONTROL_PACKET_TYPE_PUBCOMP: Byte = 7; // Publish complete (assured delivery part 3)
const MQTT_CONTROL_PACKET_TYPE_SUBSCRIBE: Byte = 8; // Client subscribe request
const MQTT_CONTROL_PACKET_TYPE_SUBACK: Byte = 9; // Subscribe acknowledgment
const MQTT_CONTROL_PACKET_TYPE_UNSUBSCRIBE: Byte = 10; // Unsubscribe request
const MQTT_CONTROL_PACKET_TYPE_UNSUBACK: Byte = 11; // Unsubscribe acknowledgment
const MQTT_CONTROL_PACKET_TYPE_PINGREQ: Byte = 12; // PING request
const MQTT_CONTROL_PACKET_TYPE_PINGRESP: Byte = 13; // PING response
const MQTT_CONTROL_PACKET_TYPE_DISCONNECT: Byte = 14; // Client is disconnecting
const MQTT_CONTROL_PACKET_TYPE_AUTH: Byte = 15; // Authentication exchange

// MQTT QoS Level
const MQTT_QOS_LEVEL_0: Byte = 0; // At most once delivery
const MQTT_QOS_LEVEL_1: Byte = 1; // At least once delivery
const MQTT_QOS_LEVEL_2: Byte = 2; // Exactly once delivery

// MQTT Property Identifier
const MQTT_PROPERTY_PAYLOAD_FORMAT_INDICATOR: Byte = 0x01; // Payload Format Indicator
const MQTT_PROPERTY_MESSAGE_EXPIRY_INTERVAL: Byte = 0x02; // Message Expiry Interval
const MQTT_PROPERTY_CONTENT_TYPE: Byte = 0x03; // Content Type
const MQTT_PROPERTY_RESPONSE_TOPIC: Byte = 0x08; // Response Topic
const MQTT_PROPERTY_CORRELATION_DATA: Byte = 0x09; // Correlation Data
const MQTT_PROPERTY_SUBSCRIPTION_IDENTIFIER: Byte = 0x0B; // Subscription Identifier
const MQTT_PROPERTY_SESSION_EXPIRY_INTERVAL: Byte = 0x11; // Session Expiry Interval
const MQTT_PROPERTY_ASSIGNED_CLIENT_IDENTIFIER: Byte = 0x12; // Assigned Client Identifier
const MQTT_PROPERTY_SERVER_KEEP_ALIVE: Byte = 0x13; // Server Keep Alive
const MQTT_PROPERTY_AUTHENTICATION_METHOD: Byte = 0x15; // Authentication Method
const MQTT_PROPERTY_AUTHENTICATION_DATA: Byte = 0x16; // Authentication Data
const MQTT_PROPERTY_REQUEST_PROBLEM_INFORMATION: Byte = 0x17; // Request Problem Information
const MQTT_PROPERTY_WILL_DELAY_INTERVAL: Byte = 0x18; // Will Delay Interval
const MQTT_PROPERTY_REQUEST_RESPONSE_INFORMATION: Byte = 0x19; // Request Response Information
const MQTT_PROPERTY_RESPONSE_INFORMATION: Byte = 0x1A; // Response Information
const MQTT_PROPERTY_SERVER_REFERENCE: Byte = 0x1C; // Server Reference
const MQTT_PROPERTY_REASON_STRING: Byte = 0x1F; // Reason String
const MQTT_PROPERTY_RECEIVE_MAXIMUM: Byte = 0x21; // Receive Maximum
const MQTT_PROPERTY_TOPIC_ALIAS_MAXIMUM: Byte = 0x22; // Topic Alias Maximum
const MQTT_PROPERTY_TOPIC_ALIAS: Byte = 0x23; // Topic Alias
const MQTT_PROPERTY_MAXIMUM_QOS: Byte = 0x24; // Maximum QoS
const MQTT_PROPERTY_RETAIN_AVAILABLE: Byte = 0x25; // Retain Available
const MQTT_PROPERTY_USER_PROPERTY: Byte = 0x26; // User Property
const MQTT_PROPERTY_MAXIMUM_PACKET_SIZE: Byte = 0x27; // Maximum Packet Size
const MQTT_PROPERTY_WILDCARD_SUBSCRIPTION_AVAILABLE: Byte = 0x28; // Wildcard Subscription Available
const MQTT_PROPERTY_SUBSCRIPTION_IDENTIFIER_AVAILABLE: Byte = 0x29; // Subscription Identifier Available
const MQTT_PROPERTY_SHARED_SUBSCRIPTION_AVAILABLE: Byte = 0x2A; // Shared Subscription Available

// MQTT Reason Code
const MQTT_REASON_CODE_SUCCESS: Byte = 0x00; // Normal disconnection
const MQTT_REASON_CODE_GRANTED_QOS0: Byte = 0x00; // Granted QoS 0
const MQTT_REASON_CODE_GRANTED_QOS1: Byte = 0x01; // Granted QoS 1
const MQTT_REASON_CODE_GRANTED_QOS2: Byte = 0x02; // Granted QoS 2
const MQTT_REASON_CODE_DISCONNECT_WITH_WILL_MESSAGE: Byte = 0x04; // Disconnect with Will Message
const MQTT_REASON_CODE_NO_MATCHING_SUBSCRIBERS: Byte = 0x10; // No matching subscribers
const MQTT_REASON_CODE_NO_SUBSCRIPTION_EXISTED: Byte = 0x11; // No subscription existed
const MQTT_REASON_CODE_CONTINUE_AUTHENTICATION: Byte = 0x18; // Continue authentication
const MQTT_REASON_CODE_RE_AUTHENTICATE: Byte = 0x19; // Re-authenticate
const MQTT_REASON_CODE_UNSPECIFIED_ERROR: Byte = 0x80; // Unspecified error
const MQTT_REASON_CODE_MALFORMED_PACKET: Byte = 0x81; // Malformed Packet
const MQTT_REASON_CODE_PROTOCOL_ERROR: Byte = 0x82; // Protocol Error
const MQTT_REASON_CODE_IMPLEMENTATION_SPECIFIC_ERROR: Byte = 0x83; // Implementation specific error
const MQTT_REASON_CODE_UNSUPPORTED_PROTOCOL_VERSION: Byte = 0x84; // Unsupported Protocol Version
const MQTT_REASON_CODE_CLIENT_IDENTIFIER_NOT_VALID: Byte = 0x85; // Client Identifier not valid
const MQTT_REASON_CODE_BAD_USER_NAME_OR_PASSWORD: Byte = 0x86; // Bad User Name or Password
const MQTT_REASON_CODE_NOT_AUTHORIZED: Byte = 0x87; // Not authorized
const MQTT_REASON_CODE_SERVER_UNAVAILABLE: Byte = 0x88; // Server unavailable
const MQTT_REASON_CODE_SERVER_BUSY: Byte = 0x89; // Server busy
const MQTT_REASON_CODE_BANNED: Byte = 0x8A; // Banned
const MQTT_REASON_CODE_SERVER_SHUTTING_DOWN: Byte = 0x8B; // Server shutting down
const MQTT_REASON_CODE_BAD_AUTHENTICATION_METHOD: Byte = 0x8C; // Bad authentication method
const MQTT_REASON_CODE_KEEP_ALIVE_TIMEOUT: Byte = 0x8D; // Keep Alive timeout
const MQTT_REASON_CODE_SESSION_TAKEN_OVER: Byte = 0x8E; // Session taken over
const MQTT_REASON_CODE_TOPIC_FILTER_INVALID: Byte = 0x8F; // Topic Filter invalid
const MQTT_REASON_CODE_TOPIC_NAME_INVALID: Byte = 0x90; // Topic Name invalid
const MQTT_REASON_CODE_PACKET_IDENTIFIER_IN_USE: Byte = 0x91; // Packet Identifier in use
const MQTT_REASON_CODE_PACKET_IDENTIFIER_NOT_FOUND: Byte = 0x92; // Packet Identifier not found
const MQTT_REASON_CODE_RECEIVE_MAXIMUM_EXCEEDED: Byte = 0x93; // Receive Maximum exceeded
const MQTT_REASON_CODE_TOPIC_ALIAS_INVALID: Byte = 0x94; // Topic Alias invalid
const MQTT_REASON_CODE_PACKET_TOO_LARGE: Byte = 0x95; // Packet too large
const MQTT_REASON_CODE_MESSAGE_RATE_TOO_HIGH: Byte = 0x96; // Message rate too high
const MQTT_REASON_CODE_QUOTA_EXCEEDED: Byte = 0x97; // Quota exceeded
const MQTT_REASON_CODE_ADMINISTRATIVE_ACTION: Byte = 0x98; // Administrative action
const MQTT_REASON_CODE_PAYLOAD_FORMAT_INVALID: Byte = 0x99; // Payload format invalid
const MQTT_REASON_CODE_RETAIN_NOT_SUPPORTED: Byte = 0x9A; // Retain not supported
const MQTT_REASON_CODE_QOS_NOT_SUPPORTED: Byte = 0x9B; // QoS not supported
const MQTT_REASON_CODE_USE_ANOTHER_SERVER: Byte = 0x9C; // Use another server
const MQTT_REASON_CODE_SERVER_MOVED: Byte = 0x9D; // Server moved
const MQTT_REASON_CODE_SHARED_SUBSCRIPTIONS_NOT_SUPPORTED: Byte = 0x9E; // Shared Subscriptions not supported
const MQTT_REASON_CODE_CONNECTION_RATE_EXCEEDED: Byte = 0x9F; // Connection rate exceeded
const MQTT_REASON_CODE_MAXIMUM_CONNECT_TIME: Byte = 0xA0; // Maximum connect time
const MQTT_REASON_CODE_SUBSCRIPTION_IDENTIFIERS_NOT_SUPPORTED: Byte = 0xA1; // Subscription Identifiers not supported
const MQTT_REASON_CODE_WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED: Byte = 0xA2; // Wildcard Subscriptions not supported

// MQTT Retain Handling Option
const MQTT_RETAIN_HANDLING_SEND_AT_SUBSCRIBE_TIME: Byte = 0x00;
const MQTT_RETAIN_HANDLING_SEND_AT_SUBSCRIBE_TIME_IF_NEW_SUBSCRIPTION: Byte = 0x01;
const MQTT_RETAIN_HANDLING_DO_NOT_SEND_AT_SUBSCRIBE_TIME: Byte = 0x02;



#[derive(Debug, Clone, Copy)]
pub enum QoSLevel {
    AtMostOnce, // At most once delivery
    AtLeastOnce, // At least once delivery
    ExactlyOnce, // Exactly once delivery
}

#[derive(Debug)]
pub enum ReasonCode {
    Success, // Normal disconnection, Granted QoS 0
    GrantedQoS0, // Granted QoS 0
    GrantedQoS1, // Granted QoS 1
    GrantedQoS2, // Granted QoS 2
    DisconnectWithWillMessage, // Disconnect with Will Message
    NoMatchingSubscribers, // No matching subscribers
    NoSubscriptionExisted, // No subscription existed
    ContinueAuthentication, // Continue authentication
    ReAuthenticate, // Re-authenticate
    UnspecifiedError, // Unspecified error
    MalformedPacket, // Malformed Packet
    ProtocolError, // Protocol Error
    ImplementationSpecificError, // Implementation specific error
    UnsupportedProtocolVersion, // Unsupported Protocol Version
    ClientIdentifierNotValid, // Client Identifier not valid
    BadUserNameOrPassword, // Bad User Name or Password
    NotAuthorized, // Not authorized
    ServerUnavailable, // Server unavailable
    ServerBusy, // Server busy
    Banned, // Banned
    ServerShuttingDown, // Server shutting down
    BadAuthenticationMethod, // Bad authentication method
    KeepAliveTimeout, // Keep Alive timeout
    SessionTakenOver, // Session taken over
    TopicFilterInvalid, // Topic Filter invalid
    TopicNameInvalid, // Topic Name invalid
    PacketIdentifierInUse, // Packet Identifier in use
    PacketIdentifierNotFound, // Packet Identifier not found
    ReceiveMaximumExeeded, // Receive Maximum exceeded
    TopicAliasInvalid, // Topic Alias invalid
    PacketTooLarge, // Packet too large
    MessageRateTooHigh, // Message rate too high
    QuotaExceeded, // Quota exceeded
    AdministrativeAction, // Administrative action
    PayloadFormatInvalid, // Payload format invalid
    RetainNotSupported, // Retain not supported
    QoSNotSupported, // QoS not supported
    UseAnotherServer, // Use another server
    ServerMoved, // Server moved
    SharedSubscriptionsNotSupported, // Shared Subscriptions not supported
    ConnectionRateExceeded, // Connection rate exceeded
    MaximumConnectTime, // Maximum connect time
    SubscriptionIdentifiersNotSupported, // Subscription Identifiers not supported
    WildcardSubscriptionsNotSupported, // Wildcard Subscriptions not supported
}

impl ReasonCode {
    pub fn from_byte(code: Byte) -> Result<ReasonCode, ReasonCode> {
        match code {
            MQTT_REASON_CODE_SUCCESS => Ok(ReasonCode::Success),
            MQTT_REASON_CODE_GRANTED_QOS1 => Ok(ReasonCode::GrantedQoS1),
            MQTT_REASON_CODE_GRANTED_QOS2 => Ok(ReasonCode::GrantedQoS2),
            MQTT_REASON_CODE_DISCONNECT_WITH_WILL_MESSAGE => Ok(ReasonCode::DisconnectWithWillMessage),
            MQTT_REASON_CODE_NO_MATCHING_SUBSCRIBERS => Ok(ReasonCode::NoMatchingSubscribers),
            MQTT_REASON_CODE_NO_SUBSCRIPTION_EXISTED => Ok(ReasonCode::NoSubscriptionExisted),
            MQTT_REASON_CODE_CONTINUE_AUTHENTICATION => Ok(ReasonCode::ContinueAuthentication),
            MQTT_REASON_CODE_RE_AUTHENTICATE => Ok(ReasonCode::ReAuthenticate),
            MQTT_REASON_CODE_UNSPECIFIED_ERROR => Ok(ReasonCode::UnspecifiedError),
            MQTT_REASON_CODE_MALFORMED_PACKET => Ok(ReasonCode::MalformedPacket),
            MQTT_REASON_CODE_PROTOCOL_ERROR => Ok(ReasonCode::ProtocolError),
            MQTT_REASON_CODE_IMPLEMENTATION_SPECIFIC_ERROR => Ok(ReasonCode::ImplementationSpecificError),
            MQTT_REASON_CODE_UNSUPPORTED_PROTOCOL_VERSION => Ok(ReasonCode::UnsupportedProtocolVersion),
            MQTT_REASON_CODE_CLIENT_IDENTIFIER_NOT_VALID => Ok(ReasonCode::ClientIdentifierNotValid),
            MQTT_REASON_CODE_BAD_USER_NAME_OR_PASSWORD => Ok(ReasonCode::BadUserNameOrPassword),
            MQTT_REASON_CODE_NOT_AUTHORIZED => Ok(ReasonCode::NotAuthorized),
            MQTT_REASON_CODE_SERVER_UNAVAILABLE => Ok(ReasonCode::ServerUnavailable),
            MQTT_REASON_CODE_SERVER_BUSY => Ok(ReasonCode::ServerBusy),
            MQTT_REASON_CODE_BANNED => Ok(ReasonCode::Banned),
            MQTT_REASON_CODE_SERVER_SHUTTING_DOWN => Ok(ReasonCode::ServerShuttingDown),
            MQTT_REASON_CODE_BAD_AUTHENTICATION_METHOD => Ok(ReasonCode::BadAuthenticationMethod),
            MQTT_REASON_CODE_KEEP_ALIVE_TIMEOUT => Ok(ReasonCode::KeepAliveTimeout),
            MQTT_REASON_CODE_SESSION_TAKEN_OVER => Ok(ReasonCode::SessionTakenOver),
            MQTT_REASON_CODE_TOPIC_FILTER_INVALID => Ok(ReasonCode::TopicFilterInvalid),
            MQTT_REASON_CODE_TOPIC_NAME_INVALID => Ok(ReasonCode::TopicNameInvalid),
            MQTT_REASON_CODE_PACKET_IDENTIFIER_IN_USE => Ok(ReasonCode::PacketIdentifierInUse),
            MQTT_REASON_CODE_PACKET_IDENTIFIER_NOT_FOUND => Ok(ReasonCode::PacketIdentifierNotFound),
            MQTT_REASON_CODE_RECEIVE_MAXIMUM_EXCEEDED => Ok(ReasonCode::ReceiveMaximumExeeded),
            MQTT_REASON_CODE_TOPIC_ALIAS_INVALID => Ok(ReasonCode::TopicAliasInvalid),
            MQTT_REASON_CODE_PACKET_TOO_LARGE => Ok(ReasonCode::PacketTooLarge),
            MQTT_REASON_CODE_MESSAGE_RATE_TOO_HIGH => Ok(ReasonCode::MessageRateTooHigh),
            MQTT_REASON_CODE_QUOTA_EXCEEDED => Ok(ReasonCode::QuotaExceeded),
            MQTT_REASON_CODE_ADMINISTRATIVE_ACTION => Ok(ReasonCode::AdministrativeAction),
            MQTT_REASON_CODE_PAYLOAD_FORMAT_INVALID => Ok(ReasonCode::PayloadFormatInvalid),
            MQTT_REASON_CODE_RETAIN_NOT_SUPPORTED => Ok(ReasonCode::RetainNotSupported),
            MQTT_REASON_CODE_QOS_NOT_SUPPORTED => Ok(ReasonCode::QoSNotSupported),
            MQTT_REASON_CODE_USE_ANOTHER_SERVER => Ok(ReasonCode::UseAnotherServer),
            MQTT_REASON_CODE_SERVER_MOVED => Ok(ReasonCode::ServerMoved),
            MQTT_REASON_CODE_SHARED_SUBSCRIPTIONS_NOT_SUPPORTED => Ok(ReasonCode::SharedSubscriptionsNotSupported),
            MQTT_REASON_CODE_CONNECTION_RATE_EXCEEDED => Ok(ReasonCode::ConnectionRateExceeded),
            MQTT_REASON_CODE_MAXIMUM_CONNECT_TIME => Ok(ReasonCode::MaximumConnectTime),
            MQTT_REASON_CODE_SUBSCRIPTION_IDENTIFIERS_NOT_SUPPORTED => Ok(ReasonCode::SubscriptionIdentifiersNotSupported),
            MQTT_REASON_CODE_WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED => Ok(ReasonCode::WildcardSubscriptionsNotSupported),
            _ => Err(ReasonCode::ProtocolError),
        }
    }
    pub fn to_byte(&self) -> Byte {
        match self {
            ReasonCode::Success => MQTT_REASON_CODE_SUCCESS,
            ReasonCode::GrantedQoS0 => MQTT_REASON_CODE_GRANTED_QOS0,
            ReasonCode::GrantedQoS1 => MQTT_REASON_CODE_GRANTED_QOS1,
            ReasonCode::GrantedQoS2 => MQTT_REASON_CODE_GRANTED_QOS2,
            ReasonCode::DisconnectWithWillMessage => MQTT_REASON_CODE_DISCONNECT_WITH_WILL_MESSAGE,
            ReasonCode::NoMatchingSubscribers => MQTT_REASON_CODE_NO_MATCHING_SUBSCRIBERS,
            ReasonCode::NoSubscriptionExisted => MQTT_REASON_CODE_NO_SUBSCRIPTION_EXISTED,
            ReasonCode::ContinueAuthentication => MQTT_REASON_CODE_CONTINUE_AUTHENTICATION,
            ReasonCode::ReAuthenticate => MQTT_REASON_CODE_RE_AUTHENTICATE,
            ReasonCode::UnspecifiedError => MQTT_REASON_CODE_UNSPECIFIED_ERROR,
            ReasonCode::MalformedPacket => MQTT_REASON_CODE_MALFORMED_PACKET,
            ReasonCode::ProtocolError => MQTT_REASON_CODE_PROTOCOL_ERROR,
            ReasonCode::ImplementationSpecificError => MQTT_REASON_CODE_IMPLEMENTATION_SPECIFIC_ERROR,
            ReasonCode::UnsupportedProtocolVersion => MQTT_REASON_CODE_UNSUPPORTED_PROTOCOL_VERSION,
            ReasonCode::ClientIdentifierNotValid => MQTT_REASON_CODE_CLIENT_IDENTIFIER_NOT_VALID,
            ReasonCode::BadUserNameOrPassword => MQTT_REASON_CODE_BAD_USER_NAME_OR_PASSWORD,
            ReasonCode::NotAuthorized => MQTT_REASON_CODE_NOT_AUTHORIZED,
            ReasonCode::ServerUnavailable => MQTT_REASON_CODE_SERVER_UNAVAILABLE,
            ReasonCode::ServerBusy => MQTT_REASON_CODE_SERVER_BUSY,
            ReasonCode::Banned => MQTT_REASON_CODE_BANNED,
            ReasonCode::ServerShuttingDown => MQTT_REASON_CODE_SERVER_SHUTTING_DOWN,
            ReasonCode::BadAuthenticationMethod => MQTT_REASON_CODE_BAD_AUTHENTICATION_METHOD,
            ReasonCode::KeepAliveTimeout => MQTT_REASON_CODE_KEEP_ALIVE_TIMEOUT,
            ReasonCode::SessionTakenOver => MQTT_REASON_CODE_SESSION_TAKEN_OVER,
            ReasonCode::TopicFilterInvalid => MQTT_REASON_CODE_TOPIC_FILTER_INVALID,
            ReasonCode::TopicNameInvalid => MQTT_REASON_CODE_TOPIC_NAME_INVALID,
            ReasonCode::PacketIdentifierInUse => MQTT_REASON_CODE_PACKET_IDENTIFIER_IN_USE,
            ReasonCode::PacketIdentifierNotFound => MQTT_REASON_CODE_PACKET_IDENTIFIER_NOT_FOUND,
            ReasonCode::ReceiveMaximumExeeded => MQTT_REASON_CODE_RECEIVE_MAXIMUM_EXCEEDED,
            ReasonCode::TopicAliasInvalid => MQTT_REASON_CODE_TOPIC_ALIAS_INVALID,
            ReasonCode::PacketTooLarge => MQTT_REASON_CODE_PACKET_TOO_LARGE,
            ReasonCode::MessageRateTooHigh => MQTT_REASON_CODE_MESSAGE_RATE_TOO_HIGH,
            ReasonCode::QuotaExceeded => MQTT_REASON_CODE_QUOTA_EXCEEDED,
            ReasonCode::AdministrativeAction => MQTT_REASON_CODE_ADMINISTRATIVE_ACTION,
            ReasonCode::PayloadFormatInvalid => MQTT_REASON_CODE_PAYLOAD_FORMAT_INVALID,
            ReasonCode::RetainNotSupported => MQTT_REASON_CODE_RETAIN_NOT_SUPPORTED,
            ReasonCode::QoSNotSupported => MQTT_REASON_CODE_QOS_NOT_SUPPORTED,
            ReasonCode::UseAnotherServer => MQTT_REASON_CODE_USE_ANOTHER_SERVER,
            ReasonCode::ServerMoved => MQTT_REASON_CODE_SERVER_MOVED,
            ReasonCode::SharedSubscriptionsNotSupported => MQTT_REASON_CODE_SHARED_SUBSCRIPTIONS_NOT_SUPPORTED,
            ReasonCode::ConnectionRateExceeded => MQTT_REASON_CODE_CONNECTION_RATE_EXCEEDED,
            ReasonCode::MaximumConnectTime => MQTT_REASON_CODE_MAXIMUM_CONNECT_TIME,
            ReasonCode::SubscriptionIdentifiersNotSupported => MQTT_REASON_CODE_SUBSCRIPTION_IDENTIFIERS_NOT_SUPPORTED,
            ReasonCode::WildcardSubscriptionsNotSupported => MQTT_REASON_CODE_WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED,
        }
    }
}


#[derive(Debug)]
pub enum Property {
    PayloadFormatIndicator(Byte), // 0 = Unspecified (default), 1 = UTF-8
    MessageExpiryInterval(FourByteInteger), // Measured in seconds
    ContentType(UTF8EncodedString), // MIME type
    ResponseTopic(UTF8EncodedString),
    CorrelationData(BinaryData),
    SubscriptionIdentifier(VariableByteInteger),
    SessionExpiryInterval(FourByteInteger),
    AssignedClientIdentifier(UTF8EncodedString),
    ServerKeepAlive(TwoByteInteger),
    AuthenticationMethod(UTF8EncodedString),
    AuthenticationData(BinaryData),
    RequestProblemInformation(Byte),
    WillDelayInterval(FourByteInteger),
    RequestResponseInformation(Byte),
    ResponseInformation(UTF8EncodedString),
    ServerReference(UTF8EncodedString),
    ReasonString(UTF8EncodedString),
    ReceiveMaximum(TwoByteInteger),
    TopicAliasMaximum(TwoByteInteger),
    TopicAlias(TwoByteInteger),
    MaximumQoS(QoSLevel),
    RetainAvailable(Byte),
    UserProperty(UTF8StringPair),
    MaximumPacketSize(FourByteInteger),
    WildcardSubscriptionAvailable(Byte),
    SubscriptionIdentifierAvailable(Byte),
    SharedSubscriptionAvailable(Byte),
}

#[derive(Debug)]
pub struct ConnectPacket {
    protocol_name: UTF8EncodedString,
    protocol_version: Byte,
    clean_start: Bit,
    will: Option<Will>,
    keep_alive: TwoByteInteger,
    session_expiry_interval: Option<FourByteInteger>,
    receive_maximum: Option<TwoByteInteger>,
    maximum_packet_size: Option<FourByteInteger>,
    topic_alias_maximum: Option<TwoByteInteger>,
    request_response_information: Option<Bit>,
    request_problem_information: Option<Bit>,
    user_properties: Option<Vec<UTF8StringPair>>,
    authentication_method: Option<UTF8EncodedString>,
    authentication_data: Option<BinaryData>,
    client_identifier: UTF8EncodedString,
    user_name: Option<UTF8EncodedString>,
    password: Option<UTF8EncodedString>,
}

#[derive(Debug)]
pub struct Will {
    qos: QoSLevel,
    retain: Bit,
    topic: UTF8EncodedString,
    will_delay_interval: Option<FourByteInteger>,
    message_expiry_interval: Option<FourByteInteger>,
    content_type: Option<UTF8EncodedString>,
    response_topic: Option<UTF8EncodedString>,
    correlation_data: Option<BinaryData>,
    user_properties: Option<Vec<UTF8StringPair>>,
    payload: PublishPayload,
}

#[derive(Debug)]
pub struct ConnAckPacket {
    session_present: Bit,
    connect_reason_code: ReasonCode,
    session_expiry_interval: Option<FourByteInteger>,
    receive_maximum: Option<TwoByteInteger>,
    maximum_qos: Option<QoSLevel>,
    retain_available: Option<Bit>,
    maximum_packet_size: Option<FourByteInteger>,
    assigned_client_identifier: Option<UTF8EncodedString>,
    topic_alias_maximum: Option<TwoByteInteger>,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
    wildcard_subscription_available: Option<Bit>,
    subscription_identifier_available: Option<Bit>,
    shared_subscription_available: Option<Bit>,
    server_keep_alive: Option<TwoByteInteger>,
    response_information: Option<UTF8EncodedString>,
    server_reference: Option<UTF8EncodedString>,
    authentication_method: Option<UTF8EncodedString>,
    authentication_data: Option<BinaryData>,
}

#[derive(Debug)]
pub enum PublishPayload {
    BinaryData(BinaryData), // payload type 0
    UTF8EncodedString(UTF8EncodedString) // payload type 1
}

#[derive(Debug)]
pub struct PublishPacket {
    dup: Bit, // 0 = Original, 1 = Duplicate
    qos: QoSLevel, // 0 = At most once, 1 = At least once, 2 = Exactly once
    retain: Bit, // 0 = No retain, 1 = Retain
    topic_name: UTF8EncodedString, // Topic name
    packet_identifier: Option<TwoByteInteger>, // Packet identifier
    message_expiry_interval: Option<FourByteInteger>, // Measured in seconds 
    topic_alias: Option<TwoByteInteger>, // Topic alias
    response_topic: Option<UTF8EncodedString>, // topic name for response
    correlation_data: Option<BinaryData>, // used in Request/Response
    user_properties: Option<Vec<UTF8StringPair>>, // User properties
    subscription_identifier: Option<VariableByteInteger>, // Subscription identifier
    content_type: Option<UTF8EncodedString>, // Content type
    payload: PublishPayload, // Payload
}

#[derive(Debug)]
pub struct PubAckPacket {
    packet_identifier: TwoByteInteger, // The Packet Identifier of the PUBLISH Packet being acknowledged.
    reason_code: ReasonCode, // Reason code.
    reason_string: Option<UTF8EncodedString>, // OPTIONAL UTF-8 string that describes the reason associated with the Reason Code.
    user_properties: Option<Vec<UTF8StringPair>>, // OPTIONAL, User Property.
}

#[derive(Debug)]
pub struct PubRecPacket {
    packet_identifier: TwoByteInteger,
    reason_code: ReasonCode,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
}

#[derive(Debug)]
pub struct PubRelPacket {
    packet_identifier: TwoByteInteger,
    reason_code: ReasonCode,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
}

#[derive(Debug)]
pub struct PubCompPacket {
    packet_identifier: TwoByteInteger,
    reason_code: ReasonCode,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
}

#[derive(Debug)]
pub enum RetainHandling {
    SendAtSubscribeTime,
    SendAtSubscribeTimeIfNewSubscription,
    DoNotSendAtSubscribeTime,
}

#[derive(Debug)]
pub struct SubscriptionItem {
    topic_filter: UTF8EncodedString,
    maximum_qos: Byte,
    no_local: Bit,
    retain_as_published: Bit,
    retain_handling: RetainHandling,
}

#[derive(Debug)]
pub struct SubscribePacket {
    packet_identifier: TwoByteInteger,
    subscription_identifier: Option<VariableByteInteger>,
    user_properties: Option<Vec<UTF8StringPair>>,
    maximum_qos: Option<Byte>,
    no_local: Option<Bit>,
    retain_as_published: Option<Bit>,
    subscriptions: Vec<SubscriptionItem>,
}

#[derive(Debug)]
pub struct SubAckPacket {
    packet_identifier: TwoByteInteger,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
    reason_codes: Vec<ReasonCode>,
}

#[derive(Debug)]
pub struct UnsubscribePacket {
    packet_identifier: TwoByteInteger,
    user_properties: Option<Vec<UTF8StringPair>>,
    topic_filters: Vec<UTF8EncodedString>,
}

#[derive(Debug)]
pub struct UnsubAckPacket {
    packet_identifier: TwoByteInteger,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
    reason_codes: Vec<ReasonCode>,
}

#[derive(Debug)]
pub struct DisconnectPacket {
    reason_code: ReasonCode,
    session_expiry_interval: Option<FourByteInteger>,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
    server_reference: Option<UTF8EncodedString>,
}

#[derive(Debug)]
pub struct AuthPacket {
    reason_code: ReasonCode,
    authentication_method: Option<UTF8EncodedString>,
    authentication_data: Option<BinaryData>,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
}

#[derive(Debug)]
pub enum ControlPacket {
    Connect(ConnectPacket),
    ConnAck(ConnAckPacket),
    Publish(PublishPacket),
    PubAck(PubAckPacket),
    PubRec(PubRecPacket),
    PubRel(PubRelPacket),
    PubComp(PubCompPacket),
    Subscribe(SubscribePacket),
    SubAck(SubAckPacket),
    Unsubscribe(UnsubscribePacket),
    UnsubAck(UnsubAckPacket),
    PingReq,
    PingResp,
    Disconnect(DisconnectPacket),
    Auth(AuthPacket),
}

#[derive(Debug)]
pub struct MqttWriter<'a>
{
    buffer: &'a mut [Byte],
    position: usize,
}

#[derive(Debug)]
pub struct MqttReader<'a>
{
    buffer: &'a [Byte],
    position: usize,
}


impl<'a> MqttWriter<'a> {
    pub fn new(buffer: &'a mut [Byte]) -> MqttWriter {
        MqttWriter {
            buffer: buffer,
            position: 0,
        }
    }

    pub fn write_byte(&mut self, byte: Byte) -> Result<(), ReasonCode> {        
        if self.position < self.buffer.len() {
            self.buffer[self.position] = byte;
            self.position += 1;
            Ok(())
        } else {
            Err(ReasonCode::PacketTooLarge)
        }
    }

    pub fn write_two_byte_integer(&mut self, integer: TwoByteInteger) -> Result<(), ReasonCode> {
        self.write_byte((integer >> 8) as Byte)?;
        self.write_byte(integer as Byte)?;
        Ok(())
    }

    pub fn write_four_byte_integer(&mut self, integer: FourByteInteger) -> Result<(), ReasonCode> {
        self.write_byte((integer >> 24) as Byte)?;
        self.write_byte((integer >> 16) as Byte)?;
        self.write_byte((integer >> 8) as Byte)?;
        self.write_byte(integer as Byte)?;
        Ok(())
    }

    pub fn write_variable_byte_integer(&mut self, integer: VariableByteInteger) -> Result<(), ReasonCode> {
        let mut value = integer;
        loop {
            let mut encoded_byte = (value % 128) as Byte;
            value /= 128;
            if value > 0 {
                encoded_byte |= 128;
            }
            self.write_byte(encoded_byte)?;
            if value <= 0 {
                break;
            }
        }
        Ok(())
    }

    pub fn write_utf8_string(&mut self, string: &UTF8EncodedString) -> Result<(), ReasonCode> {
        self.write_two_byte_integer(string.len() as TwoByteInteger)?;
        for byte in string.bytes() {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    pub fn write_utf8_string_pair(&mut self, pair: &UTF8StringPair) -> Result<(), ReasonCode> {
        self.write_utf8_string(&pair.0)?;
        self.write_utf8_string(&pair.1)?;
        Ok(())
    }

    pub fn write_binary_data(&mut self, data: &BinaryData) -> Result<(), ReasonCode> {
        self.write_two_byte_integer(data.len() as TwoByteInteger)?;
        data.iter().try_for_each(|byte| self.write_byte(*byte))?;
        Ok(())
    }
}


impl<'a> MqttReader<'a> {
    pub fn new(buffer: &'a [Byte]) -> MqttReader<'a> {
        MqttReader {
            buffer: buffer,
            position: 0,
        }
    }

    pub fn read_byte(&mut self) -> Result<Byte, ReasonCode> {
        if self.position < self.buffer.len() {
            let byte = self.buffer[self.position];
            self.position += 1;
            Ok(byte)
        } else {
            Err(ReasonCode::MalformedPacket)
        }
    }

    pub fn read_two_byte_integer(&mut self) -> Result<TwoByteInteger, ReasonCode> {
        let msb = self.read_byte()?;
        let lsb = self.read_byte()?;
        Ok(((msb as TwoByteInteger) << 8) | (lsb as TwoByteInteger))
    }

    pub fn read_four_byte_integer(&mut self) -> Result<FourByteInteger, ReasonCode> {
        let msb = self.read_two_byte_integer()?;
        let lsb = self.read_two_byte_integer()?;
        Ok(((msb as FourByteInteger) << 16) | (lsb as FourByteInteger))
    }

    pub fn read_variable_byte_integer(&mut self) -> Result<VariableByteInteger, ReasonCode> {
        let mut value = 0;
        let mut multiplier = 1;
        loop {
            let encoded_byte = self.read_byte()?;
            value += (encoded_byte & 127) as VariableByteInteger * multiplier;
            multiplier *= 128;
            if multiplier > 128 * 128 * 128 {
                return Err(ReasonCode::MalformedPacket);
            }
            if (encoded_byte & 128) == 0 {
                break;
            }
        }
        Ok(value)
    }

    pub fn read_utf8_string(&mut self) -> Result<UTF8EncodedString, ReasonCode> {
        let length: usize = self.read_two_byte_integer()? as usize;
        let mut bytes = vec![0; length];
        for i in 0..length {
            bytes[i as usize] = self.read_byte()?;
        }
        match UTF8EncodedString::from_utf8(bytes.to_vec()) {
            Err(error) => return Err(ReasonCode::MalformedPacket),
            Ok(string) => return Ok(string),
        }
    }

    pub fn read_utf8_string_pair(&mut self) -> Result<UTF8StringPair, ReasonCode> {
        let key = self.read_utf8_string()?;
        let value = self.read_utf8_string()?;
        Ok((key, value))
    }

    pub fn read_binary_data(&mut self) -> Result<BinaryData, ReasonCode> {
        let length: usize = self.read_two_byte_integer()? as usize;
        let mut bytes = vec![0; length];
        for i in 0..length {
            bytes[i as usize] = self.read_byte()?;
        }
        Ok(bytes)
    }
}


trait BitReader {
    fn read_bit(&self, index: Byte) -> Result<bool, ReasonCode>;
}

impl BitReader for Byte {
    fn read_bit(&self, index: Byte) -> Result<bool, ReasonCode> {
        if index > 7 {
            return Err(ReasonCode::MalformedPacket);
        }

        Ok((*self & (1 << index)) != 0)
    }
}


fn sizeof_variable_byte_integer(value: VariableByteInteger) -> usize {
    let mut value = value;
    let mut size = 0;
    loop {
        size += 1;
        value /= 128;
        if value <= 0 {
            break;
        }
    }
    size
}

impl ConnectPacket {
    pub fn new_v5(client_identifier: UTF8EncodedString) -> ConnectPacket {
        ConnectPacket {
            protocol_name: String::from("MQTT"),
            protocol_version: 5,
            clean_start: true,
            will: None,
            keep_alive: 60,
            session_expiry_interval: None,
            receive_maximum: None,
            maximum_packet_size: None,
            topic_alias_maximum: None,
            request_response_information: None,
            request_problem_information: None,
            user_properties: None,
            authentication_method: None,
            authentication_data: None,
            client_identifier: client_identifier,
            user_name: None,
            password: None,
        }
    }

    pub fn new_v311(client_identifier: UTF8EncodedString) -> ConnectPacket {
        ConnectPacket {
            protocol_name: String::from("MQTT"),
            protocol_version: 4,
            clean_start: true,
            will: None,
            keep_alive: 60,
            session_expiry_interval: None,
            receive_maximum: None,
            maximum_packet_size: None,
            topic_alias_maximum: None,
            request_response_information: None,
            request_problem_information: None,
            user_properties: None,
            authentication_method: None,
            authentication_data: None,
            client_identifier: client_identifier,
            user_name: None,
            password: None,
        }
    }

    fn calculate_property_length(&self) -> usize {
        let mut length = 0;
        if let Some(_) = self.session_expiry_interval {
            length += 5;
        }
        if let Some(_) = self.receive_maximum {
            length += 3;
        }
        if let Some(_) = self.maximum_packet_size {
            length += 5;
        }
        if let Some(_) = self.topic_alias_maximum {
            length += 3;
        }
        if let Some(_) = self.request_response_information {
            length += 2;
        }
        if let Some(_) = self.request_problem_information {
            length += 2;
        }
        if let Some(p) = &self.user_properties {
            for (key, value) in p {
                length += key.len() + 2;
                length += value.len() + 2;
            }
        }
        if let Some(a) = &self.authentication_method {
            length += a.len() + 2;
        }
        if let Some(d) = &self.authentication_data {
            length += d.len() + 2;
        }
        length
    }

    fn calculate_variable_header_length(&self) -> (usize, usize) {
        let mut length = 0;
        length += self.protocol_name.len() + 2; // protocol name length
        length += 1; // protocol version
        length += 1; // connect flags
        length += 2; // keep alive
        let property_length = self.calculate_property_length();
        length += sizeof_variable_byte_integer(property_length as u32);
        length += property_length;
        (length, property_length)
    }

    fn calculate_payload_length(&self) -> usize {
        let mut length = 0;
        length += self.client_identifier.len() + 2;
        if let Some(w) = &self.will {
            length += w.topic.len() + 2;
            match &w.payload {
                PublishPayload::UTF8EncodedString(s) => length += s.len() + 2,
                PublishPayload::BinaryData(b) => length += b.len() + 2,
            }
        }
        if let Some(u) = &self.user_name {
            length += u.len() + 2;
        }
        if let Some(p) = &self.password {
            length += p.len() + 2;
        }
        length
    }

    pub fn write_to_buffer(&self, buffer: &mut [Byte]) -> Result<usize, ReasonCode> {
        let mut writer = MqttWriter::new(buffer);
        writer.write_byte(MQTT_CONTROL_PACKET_TYPE_CONNECT << 4)?;
        let (variable_header_length, property_length) = self.calculate_variable_header_length();
        let payload_length = self.calculate_payload_length();
        let remaining_length = variable_header_length + payload_length;
        writer.write_variable_byte_integer(remaining_length as VariableByteInteger)?;
        writer.write_utf8_string(&self.protocol_name)?;
        writer.write_byte(self.protocol_version)?;
        let mut connect_flags = 0;
        if self.clean_start {
            connect_flags |= 0x02;
        }
        if let Some(w) = &self.will {
            connect_flags |= 0x04;
            connect_flags |= (w.qos as Byte) << 3;
            if w.retain {
                connect_flags |= 0x20;
            }
        }
        if let Some(u) = &self.user_name {
            connect_flags |= 0x80;
            if let Some(_) = &self.password {
                connect_flags |= 0x40;
            }
        }
        writer.write_byte(connect_flags)?;
        writer.write_two_byte_integer(self.keep_alive)?;

        // Write properties
        writer.write_variable_byte_integer(property_length as VariableByteInteger)?;
        
        if let Some(s) = &self.session_expiry_interval {
            writer.write_byte(MQTT_PROPERTY_SESSION_EXPIRY_INTERVAL)?;
            writer.write_four_byte_integer(*s)?;
        }
        if let Some(r) = &self.receive_maximum {
            writer.write_byte(MQTT_PROPERTY_RECEIVE_MAXIMUM)?;
            writer.write_two_byte_integer(*r)?;
        }
        if let Some(m) = &self.maximum_packet_size {
            writer.write_byte(MQTT_PROPERTY_MAXIMUM_PACKET_SIZE)?;
            writer.write_four_byte_integer(*m)?;
        }
        if let Some(t) = &self.topic_alias_maximum {
            writer.write_byte(MQTT_PROPERTY_TOPIC_ALIAS_MAXIMUM)?;
            writer.write_two_byte_integer(*t)?;
        }
        if let Some(r) = &self.request_response_information {
            writer.write_byte(MQTT_PROPERTY_REQUEST_RESPONSE_INFORMATION)?;
            if *r {
                writer.write_byte(0x01)?;
            } else {
                writer.write_byte(0x00)?;
            }
        }
        if let Some(r) = &self.request_problem_information {
            writer.write_byte(MQTT_PROPERTY_REQUEST_PROBLEM_INFORMATION)?;
            if *r {
                writer.write_byte(0x01)?;
            } else {
                writer.write_byte(0x00)?;
            }
        }
        if let Some(p) = &self.user_properties {
            for (key, value) in p {
                writer.write_byte(MQTT_PROPERTY_USER_PROPERTY)?;
                writer.write_utf8_string(key)?;
                writer.write_utf8_string(value)?;
            }
        }
        if let Some(a) = &self.authentication_method {
            writer.write_byte(MQTT_PROPERTY_AUTHENTICATION_METHOD)?;
            writer.write_utf8_string(a)?;
        }
        if let Some(d) = &self.authentication_data {
            writer.write_byte(MQTT_PROPERTY_AUTHENTICATION_DATA)?;
            writer.write_binary_data(d)?;
        }



        Ok(0)
    }
}
