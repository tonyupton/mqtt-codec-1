type Bit = bool;
type Byte = u8;
type TwoByteInteger = u16;
type FourByteInteger = u32;
type VariableByteInteger = u32;
type UTF8EncodedString = String;
type BinaryData = Vec<u8>;
type UTF8StringPair = (String, String);


// This enum is used to identify the type of the property.
// The value of the property is stored in the Property enum.
enum PropertyIdentifier {
    PayloadFormatIndicator = 0x01,
    MessageExpiryInterval = 0x02,
    ContentType = 0x03,
    ResponseTopic = 0x08,
    CorrelationData = 0x09,
    SubscriptionIdentifier = 0x0B,
    SessionExpiryInterval = 0x11,
    AssignedClientIdentifier = 0x12,
    ServerKeepAlive = 0x13,
    AuthenticationMethod = 0x15,
    AuthenticationData = 0x16,
    RequestProblemInformation = 0x17,
    WillDelayInterval = 0x18,
    RequestResponseInformation = 0x19,
    ResponseInformation = 0x1A,
    ServerReference = 0x1C,
    ReasonString = 0x1F,
    ReceiveMaximum = 0x21,
    TopicAliasMaximum = 0x22,
    TopicAlias = 0x23,
    MaximumQoS = 0x24,
    RetainAvailable = 0x25,
    UserProperty = 0x26,
    MaximumPacketSize = 0x27,
    WildcardSubscriptionAvailable = 0x28,
    SubscriptionIdentifierAvailable = 0x29,
    SharedSubscriptionAvailable = 0x2A,
}

pub enum Property {
    PayloadFormatIndicator(Byte),
    MessageExpiryInterval(FourByteInteger),
    ContentType(UTF8EncodedString),
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
    MaximumQoS(Byte),
    RetainAvailable(Byte),
    UserProperty(UTF8StringPair),
    MaximumPacketSize(FourByteInteger),
    WildcardSubscriptionAvailable(Byte),
    SubscriptionIdentifierAvailable(Byte),
    SharedSubscriptionAvailable(Byte),
}

enum ControlPacketType {
    CONNECT = 1,
    CONNACK = 2,
    PUBLISH = 3,
    PUBACK = 4,
    PUBREC = 5,
    PUBREL = 6,
    PUBCOMP = 7,
    SUBSCRIBE = 8,
    SUBACK = 9,
    UNSUBSCRIBE = 10,
    UNSUBACK = 11,
    PINGREQ = 12,
    PINGRESP = 13,
    DISCONNECT = 14,
    AUTH = 15,
}



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

pub struct Will {
    qos: Byte,
    retain: Bit,
    topic: UTF8EncodedString,
    will_delay_interval: Option<FourByteInteger>,
    payload_format_indicator: Option<Byte>,
    message_expiry_interval: Option<FourByteInteger>,
    content_type: Option<UTF8EncodedString>,
    response_topic: Option<UTF8EncodedString>,
    correlation_data: Option<BinaryData>,
    user_properties: Option<Vec<UTF8StringPair>>,
    payload: BinaryData,
}


pub struct ConnAckPacket {
    session_present: Bit,
    connect_reason_code: ConnectReasonCode,
    session_expiry_interval: Option<FourByteInteger>,
    receive_maximum: Option<TwoByteInteger>,
    maximum_qos: Option<Byte>,
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
pub struct PublishPacket {}
pub struct PubAckPacket {}
pub struct PubRecPacket {}
pub struct PubRelPacket {}
pub struct PubCompPacket {}
pub struct SubscribePacket {}
pub struct SubAckPacket {}
pub struct UnsubscribePacket {}
pub struct UnsubAckPacket {}
pub struct PingReqPacket {}
pub struct PingRespPacket {}
pub struct DisconnectPacket {}
pub struct AuthPacket {}

pub enum ReasonCode {
    Success = 0x00, // Normal disconnection, Granted QoS 0
    GrantedQoS1 = 0x01,
    GrantedQoS2 = 0x02,
    DisconnectWithWillMessage = 0x04,
    NoMatchingSubscribers = 0x10,
    NoSubscriptionExisted = 0x11,
    ContinueAuthentication = 0x18,
    ReAuthenticate = 0x19,
    UnspecifiedError = 0x80,
    MalformedPacket = 0x81,
    ProtocolError = 0x82,
    ImplementationSpecificError = 0x83,
    UnsupportedProtocolVersion = 0x84,
    ClientIdentifierNotValid = 0x85,
    BadUserNameOrPassword = 0x86,
    NotAuthorized = 0x87,
    ServerUnavailable = 0x88,
    ServerBusy = 0x89,
    Banned = 0x8A,
    ServerShuttingDown = 0x8B,
    BadAuthenticationMethod = 0x8C,
    KeepAliveTimeout = 0x8D,
    SessionTakenOver = 0x8E,
    TopicFilterInvalid = 0x8F,
    TopicNameInvalid = 0x90,
    PacketIdentifierInUse = 0x91,
    PacketIdentifierNotFound = 0x92,
    ReceiveMaximumExeeded = 0x93,
    TopicAliasInvalid = 0x94,
    PacketTooLarge = 0x95,
    MessageRateTooHigh = 0x96,
    QuotaExceeded = 0x97,
    AdministrativeAction = 0x98,
    PayloadFormatInvalid = 0x99,
    RetainNotSupported = 0x9A,
    QoSNotSupported = 0x9B,
    UseAnotherServer = 0x9C,
    ServerMoved = 0x9D,
    SharedSubscriptionsNotSupported = 0x9E,
    ConnectionRateExceeded = 0x9F,
    MaximumConnectTime = 0xA0,
    SubscriptionIdentifiersNotSupported = 0xA1,
    WildcardSubscriptionsNotSupported = 0xA2,
}


pub enum ConnectReasonCode {
    Success = 0x00,
    UnspecifiedError = 0x80,
    MalformedPacket = 0x81,
    ProtocolError = 0x82,
    ImplementationSpecificError = 0x83,
    UnsupportedProtocolVersion = 0x84,
    ClientIdentifierNotValid = 0x85,
    BadUserNameOrPassword = 0x86,
    NotAuthorized = 0x87,
    ServerUnavailable = 0x88,
    ServerBusy = 0x89,
    Banned = 0x8A,
    BadAuthenticationMethod = 0x8C,
    TopicNameInvalid = 0x90,
    PacketTooLarge = 0x95,
    QuotaExceeded = 0x97,
    PayloadFormatInvalid = 0x99,
    RetainNotSupported = 0x9A,
    QoSNotSupported = 0x9B,
    UseAnotherServer = 0x9C,
    ServerMoved = 0x9D,
    ConnectionRateExceeded = 0x9F
}




pub enum Packet {
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
    PingReq(PingReqPacket),
    PingResp(PingRespPacket),
    Disconnect(DisconnectPacket),
    Auth(AuthPacket),
}

pub fn decode_packet() -> Result<Packet, ReasonCode> {
    Err(ReasonCode::ServerUnavailable)
}