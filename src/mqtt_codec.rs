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

#[derive(Debug)]
// MQTT Control Packet
enum ControlPacketType {
    CONNECT = 1, // Client request to connect to Server
    CONNACK = 2, // Connect acknowledgment
    PUBLISH = 3, // Publish message
    PUBACK = 4, // Publish acknowledgment
    PUBREC = 5, // Publish received (assured delivery part 1)
    PUBREL = 6, // Publish release (assured delivery part 2)
    PUBCOMP = 7, // Publish complete (assured delivery part 3)
    SUBSCRIBE = 8, // Client subscribe request
    SUBACK = 9, // Subscribe acknowledgment
    UNSUBSCRIBE = 10, // Unsubscribe request
    UNSUBACK = 11, // Unsubscribe acknowledgment
    PINGREQ = 12, // PING request
    PINGRESP = 13, // PING response
    DISCONNECT = 14, // Client is disconnecting
    AUTH = 15, // Authentication exchange
}

#[derive(Debug)]
pub enum QoSLvl {
    QoS0 = 0, // At most once delivery
    QoS1 = 1, // At least once delivery
    QoS2 = 2, // Exactly once delivery
}

#[derive(Debug)]
pub enum ReasonCode {
    Success = 0x00, // Normal disconnection, Granted QoS 0
    GrantedQoS1 = 0x01, // Granted QoS 1
    GrantedQoS2 = 0x02, // Granted QoS 2
    DisconnectWithWillMessage = 0x04, // Disconnect with Will Message
    NoMatchingSubscribers = 0x10, // No matching subscribers
    NoSubscriptionExisted = 0x11, // No subscription existed
    ContinueAuthentication = 0x18, // Continue authentication
    ReAuthenticate = 0x19, // Re-authenticate
    UnspecifiedError = 0x80, // Unspecified error
    MalformedPacket = 0x81, // Malformed Packet
    ProtocolError = 0x82, // Protocol Error
    ImplementationSpecificError = 0x83, // Implementation specific error
    UnsupportedProtocolVersion = 0x84, // Unsupported Protocol Version
    ClientIdentifierNotValid = 0x85, // Client Identifier not valid
    BadUserNameOrPassword = 0x86, // Bad User Name or Password
    NotAuthorized = 0x87, // Not authorized
    ServerUnavailable = 0x88, // Server unavailable
    ServerBusy = 0x89, // Server busy
    Banned = 0x8A, // Banned
    ServerShuttingDown = 0x8B, // Server shutting down
    BadAuthenticationMethod = 0x8C, // Bad authentication method
    KeepAliveTimeout = 0x8D, // Keep Alive timeout
    SessionTakenOver = 0x8E, // Session taken over
    TopicFilterInvalid = 0x8F, // Topic Filter invalid
    TopicNameInvalid = 0x90, // Topic Name invalid
    PacketIdentifierInUse = 0x91, // Packet Identifier in use
    PacketIdentifierNotFound = 0x92, // Packet Identifier not found
    ReceiveMaximumExeeded = 0x93, // Receive Maximum exceeded
    TopicAliasInvalid = 0x94, // Topic Alias invalid
    PacketTooLarge = 0x95, // Packet too large
    MessageRateTooHigh = 0x96, // Message rate too high
    QuotaExceeded = 0x97, // Quota exceeded
    AdministrativeAction = 0x98, // Administrative action
    PayloadFormatInvalid = 0x99, // Payload format invalid
    RetainNotSupported = 0x9A, // Retain not supported
    QoSNotSupported = 0x9B, // QoS not supported
    UseAnotherServer = 0x9C, // Use another server
    ServerMoved = 0x9D, // Server moved
    SharedSubscriptionsNotSupported = 0x9E, // Shared Subscriptions not supported
    ConnectionRateExceeded = 0x9F, // Connection rate exceeded
    MaximumConnectTime = 0xA0, // Maximum connect time
    SubscriptionIdentifiersNotSupported = 0xA1, // Subscription Identifiers not supported
    WildcardSubscriptionsNotSupported = 0xA2, // Wildcard Subscriptions not supported
}

#[derive(Debug)]
pub enum ConnectReasonCode {
    Success = 0x00, // Connection accepted
    UnspecifiedError = 0x80, // The Server does not wish to reveal the reason for the failure, or none of the other Reason Codes apply.
    MalformedPacket = 0x81, // Data within the CONNECT packet could not be correctly parsed.
    ProtocolError = 0x82, // The CONNECT packet does not conform to this specification.
    ImplementationSpecificError = 0x83, // The CONNECT packet is valid but is not accepted by this Server.
    UnsupportedProtocolVersion = 0x84, // The Server does not support the version of the MQTT protocol requested by the Client.
    ClientIdentifierNotValid = 0x85, // The Client identifier is a valid string but is not allowed by the Server.
    BadUserNameOrPassword = 0x86, // The Server does not accept the User Name or Password specified by the Client.
    NotAuthorized = 0x87, // The Client is not authorized to connect.
    ServerUnavailable = 0x88, // The MQTT Server is not available.
    ServerBusy = 0x89, // The Server is busy. Try again later.
    Banned = 0x8A, // This Client has been banned by administrative action. Contact the server administrator.
    BadAuthenticationMethod = 0x8C, // The authentication method is not supported or does not match the authentication method currently in use.
    TopicNameInvalid = 0x90, // The Will Topic Name is not malformed, but is not accepted by this Server.
    PacketTooLarge = 0x95, // The CONNECT packet exceeds the maximum permissible size.
    QuotaExceeded = 0x97, // An implementation or administrative imposed limit has been exceeded.
    PayloadFormatInvalid = 0x99, // The Will Payload does not match the specified Payload Format Indicator.
    RetainNotSupported = 0x9A, // The Server does not support retained messages, and Will Retain was set to 1.
    QoSNotSupported = 0x9B, // The Server does not support the QoS set in Will QoS.
    UseAnotherServer = 0x9C, // The Client should temporarily use another server.
    ServerMoved = 0x9D, // The Client should permanently use another server.
    ConnectionRateExceeded = 0x9F // The connection rate limit has been exceeded.
}


// This enum is used to identify the type of the property.
// The value of the property is stored in the Property enum.
#[derive(Debug)]
enum PropertyIdentifier {
    PayloadFormatIndicator = 0x01, // 0 = Unspecified (default), 1 = UTF-8
    MessageExpiryInterval = 0x02, // Measured in seconds
    ContentType = 0x03, // MIME type
    ResponseTopic = 0x08, // topic name for response
    CorrelationData = 0x09, // used in Request/Response
    SubscriptionIdentifier = 0x0B, // identifies the subscription
    SessionExpiryInterval = 0x11, // Measured in seconds
    AssignedClientIdentifier = 0x12, // Assigned Client Identifier
    ServerKeepAlive = 0x13, // Measured in seconds
    AuthenticationMethod = 0x15, // Authentication method
    AuthenticationData = 0x16, // Authentication data
    RequestProblemInformation = 0x17, // 0 = Do not send, 1 = Send
    WillDelayInterval = 0x18, // Measured in seconds
    RequestResponseInformation = 0x19, // 0 = Do not send, 1 = Send
    ResponseInformation = 0x1A, // response information
    ServerReference = 0x1C, // server reference
    ReasonString = 0x1F, // human readable string
    ReceiveMaximum = 0x21, // The Client uses this value to limit the number of QoS 1 and QoS 2 publications that it is willing to process concurrently.
    TopicAliasMaximum = 0x22, // The Client uses this value to limit the number of Topic Aliases that it is willing to hold on this Connection.
    TopicAlias = 0x23, // The Topic Alias is a non-negative integer value that is sent instead of a Topic Name in any PUBLISH packet that contains a non-empty Topic Alias field.
    MaximumQoS = 0x24, // The Client uses this value to limit the maximum QoS level at which it is willing to publish messages.
    RetainAvailable = 0x25, // 0 = Retain is not supported, 1 = Retain is supported
    UserProperty = 0x26, // UTF-8 string pair
    MaximumPacketSize = 0x27, // This value represents the maximum packet size that the Server is willing to accept.
    WildcardSubscriptionAvailable = 0x28, // 0 = Wildcard Subscriptions are not supported, 1 = Wildcard Subscriptions are supported
    SubscriptionIdentifierAvailable = 0x29, // 0 = Subscription Identifiers are not supported, 1 = Subscription Identifiers are supported
    SharedSubscriptionAvailable = 0x2A, // 0 = Shared Subscriptions are not supported, 1 = Shared Subscriptions are supported
}

#[derive(Debug)]
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
    qos: QoSLvl,
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
    connect_reason_code: ConnectReasonCode,
    session_expiry_interval: Option<FourByteInteger>,
    receive_maximum: Option<TwoByteInteger>,
    maximum_qos: Option<QoSLvl>,
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
    qos: QoSLvl, // 0 = At most once, 1 = At least once, 2 = Exactly once
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
pub enum PubAckReasonCode {
    Success = 0x00, // The message is accepted
    NoMatchingSubscribers = 0x10, // The message is accepted but there are no subscribers
    UnspecifiedError = 0x80, // The receiver does not accept the publish but either does not want to reveal the reason, or it does not match one of the other values.
    ImplementationSpecificError = 0x83, // The PUBLISH is valid but the receiver is not willing to accept it.
    NotAuthorized = 0x87, // The PUBLISH is not authorized.
    TopicNameInvalid = 0x90, // The topic name is not malformed, but is not accepted by this Client or Server.
    PacketIdentifierInUse = 0x91, // The Packet Identifier is already in use. This might indicate a mismatch in the Session State between the Client and Server.
    QuotaExceeded = 0x97, // An implementation or administrative imposed limit has been exceeded.
    PayloadFormatInvalid = 0x99, // The payload format does not match the one specified in the Payload Format Indicator.
}

#[derive(Debug)]
pub struct PubAckPacket {
    packet_identifier: TwoByteInteger, // The Packet Identifier of the PUBLISH Packet being acknowledged.
    reason_code: PubAckReasonCode, // Reason code.
    reason_string: Option<UTF8EncodedString>, // OPTIONAL UTF-8 string that describes the reason associated with the Reason Code.
    user_properties: Option<Vec<UTF8StringPair>>, // OPTIONAL, User Property.
}

#[derive(Debug)]
pub enum PubRecReasonCode {
    Success = 0x00, // The message is accepted
    NoMatchingSubscribers = 0x10, // The message is accepted but there are no subscribers
    UnspecifiedError = 0x80, // The receiver does not accept the publish but either does not want to reveal the reason, or it does not match one of the other values.
    ImplementationSpecificError = 0x83, // The PUBLISH is valid but the receiver is not willing to accept it.
    NotAuthorized = 0x87, // The PUBLISH is not authorized.
    TopicNameInvalid = 0x90, // The topic name is not malformed, but is not accepted by this Client or Server.
    PacketIdentifierInUse = 0x91, // The Packet Identifier is already in use. This might indicate a mismatch in the Session State between the Client and Server.
    QuotaExceeded = 0x97, // An implementation or administrative imposed limit has been exceeded.
    PayloadFormatInvalid = 0x99, // The payload format does not match the one specified in the Payload Format Indicator.
}

#[derive(Debug)]
pub struct PubRecPacket {
    packet_identifier: TwoByteInteger,
    reason_code: PubAckReasonCode,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
}

#[derive(Debug)]
pub enum PubRelReasonCode {
    Success = 0x00, // The message is accepted
    PacketIdentifierNotFound = 0x92, // The Packet Identifier is not known. This is not an error during recovery, but at other times indicates a mismatch between the Session State on the Client and Server.
}

#[derive(Debug)]
pub struct PubRelPacket {
    packet_identifier: TwoByteInteger,
    reason_code: PubRelReasonCode,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
}

#[derive(Debug)]
pub enum PubCompReasonCode {
    Success = 0x00, // The message is accepted
    PacketIdentifierNotFound = 0x92, // The Packet Identifier is not known. This is not an error during recovery, but at other times indicates a mismatch between the Session State on the Client and Server.
}

#[derive(Debug)]
pub struct PubCompPacket {
    packet_identifier: TwoByteInteger,
    reason_code: PubCompReasonCode,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
}

#[derive(Debug)]
pub enum RetainHandling {
    SendAtSubscribeTime = 0x00,
    SendAtSubscribeTimeIfNewSubscription = 0x01,
    DoNotSendAtSubscribeTime = 0x02,
}

#[derive(Debug)]
pub struct SubscriptionItem {
    topic_filter: UTF8EncodedString,
    maximum_qos: QoSLvl,
    no_local: Bit,
    retain_as_published: Bit,
    retain_handling: RetainHandling,
}

#[derive(Debug)]
pub struct SubscribePacket {
    packet_identifier: TwoByteInteger,
    subscription_identifier: Option<VariableByteInteger>,
    user_properties: Option<Vec<UTF8StringPair>>,
    maximum_qos: Option<QoSLvl>,
    no_local: Option<Bit>,
    retain_as_published: Option<Bit>,
    subscriptions: Vec<SubscriptionItem>,
}

#[derive(Debug)]
pub enum SubscribeReasonCode {
    GrantedQoS0 = 0x00, // Success - Maximum QoS 0
    GrantedQoS1 = 0x01, // Success - Maximum QoS 1
    GrantedQoS2 = 0x02, // Success - Maximum QoS 2
    UnspecifiedError = 0x80, // Unspecified error
    ImplementationSpecificError = 0x83, // Implementation specific error
    NotAuthorized = 0x87, // Not authorized
    TopicFilterInvalid = 0x8F, // Topic Filter invalid
    PacketIdentifierInUse = 0x91, // Packet Identifier in use
    QuotaExceeded = 0x97, // Quota exceeded
    SharedSubscriptionsNotSupported = 0x9E, // Shared Subscriptions not supported
    SubscriptionIdentifiersNotSupported = 0xA1, // Subscription Identifiers not supported
    WildcardSubscriptionsNotSupported = 0xA2, // Wildcard Subscriptions not supported
}

#[derive(Debug)]
pub struct SubAckPacket {
    packet_identifier: TwoByteInteger,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
    reason_codes: Vec<SubscribeReasonCode>,
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
    reason_codes: Vec<SubscribeReasonCode>,
}

#[derive(Debug)]
pub enum DisconnectReasonCode {
    NormalDisconnection = 0x00, // Disconnect with Will Message
    DisconnectWithWillMessage = 0x04, // Disconnect with Will Message
    UnspecifiedError = 0x80, // Unspecified error
    MalformedPacket = 0x81, // Malformed Packet
    ProtocolError = 0x82, // Protocol Error
    ImplementationSpecificError = 0x83, // Implementation specific error
    NotAuthorized = 0x87, // Not authorized
    ServerBusy = 0x89, // Server busy
    ServerShuttingDown = 0x8B, // Server shutting down
    KeepAliveTimeout = 0x8D, // Keep Alive timeout
    SessionTakenOver = 0x8E, // Session taken over
    TopicFilterInvalid = 0x8F, // Topic Filter invalid
    TopicNameInvalid = 0x90, // Topic Name invalid
    ReceiveMaximumExceeded = 0x93, // Receive Maximum exceeded
    TopicAliasInvalid = 0x94, // Topic Alias invalid
    PacketTooLarge = 0x95, // Packet too large
    MessageRateTooHigh = 0x96, // Message rate too high
    QuotaExceeded = 0x97, // Quota exceeded
    AdministrativeAction = 0x98, // Administrative action
    PayloadFormatInvalid = 0x99, // Payload format invalid
    RetainNotSupported = 0x9A, // Retain not supported
    QoSNotSupported = 0x9B, // QoS not supported
    UseAnotherServer = 0x9C, // Use another server
    ServerMoved = 0x9D, // Server moved
    SharedSubscriptionsNotSupported = 0x9E, // Shared Subscriptions not supported
    ConnectionRateExceeded = 0x9F, // Connection rate exceeded
    MaximumConnectTime = 0xA0, // Maximum connect time
    SubscriptionIdentifiersNotSupported = 0xA1, // Subscription Identifiers not supported
    WildcardSubscriptionsNotSupported = 0xA2, // Wildcard Subscriptions not supported
}

#[derive(Debug)]
pub struct DisconnectPacket {
    reason_code: DisconnectReasonCode,
    session_expiry_interval: Option<FourByteInteger>,
    reason_string: Option<UTF8EncodedString>,
    user_properties: Option<Vec<UTF8StringPair>>,
    server_reference: Option<UTF8EncodedString>,
}

#[derive(Debug)]
pub enum AuthReasonCode {
    Success = 0x00, // Success
    ContinueAuthentication = 0x18, // Continue authentication
    ReAuthenticate = 0x19, // Re-authenticate
}

#[derive(Debug)]
pub struct AuthPacket {
    reason_code: AuthReasonCode,
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

pub fn decode_packet() -> Result<ControlPacket, ReasonCode> {
    Ok(ControlPacket::Connect(ConnectPacket {
        protocol_name: "MQTT".to_string(),
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
        client_identifier: "Client 1".to_string(),
        user_name: None,
        password: None,
    }))
}

pub struct PacketDecoder<'a> {
    buffer: &'a[u8],
    position: u32,
}

pub struct PacketEncoder<'a> {
    buffer: &'a mut[u8],
    position: u32,
}

trait BitReader {
    fn read_bit(&self, index: u8) -> Result<bool, ReasonCode>;
}

impl BitReader for Byte {
    fn read_bit(&self, index: u8) -> Result<bool, ReasonCode> {
        if index > 7 {
            return Err(ReasonCode::MalformedPacket);
        }

        Ok((*self & (1 << index)) != 0)
    }
}

impl PacketDecoder<'_> {
    pub fn new(buffer: &[u8]) -> PacketDecoder {
        PacketDecoder {
            buffer,
            position: 0,
        }
    }

    fn read_byte(&mut self) -> Result<Byte, ReasonCode> {
        if self.position >= self.buffer.len() as u32 {
            return Err(ReasonCode::MalformedPacket);
        }

        let byte = self.buffer[self.position as usize];
        self.position += 1;

        let bit = byte.read_bit(1)?;

        Ok(byte)
    }

    fn read_two_byte_integer(&mut self) -> Result<TwoByteInteger, ReasonCode> {
        let msb = self.read_byte()? as u16;
        let lsb = self.read_byte()? as u16;
        Ok((msb << 8) | lsb)
    }

    fn read_four_byte_integer(&mut self) -> Result<FourByteInteger, ReasonCode> {
        let msb = self.read_byte()? as u32;
        let byte2 = self.read_byte()? as u32;
        let byte3 = self.read_byte()? as u32;
        let lsb = self.read_byte()? as u32;
        Ok((msb << 24) | (byte2 << 16) | (byte3 << 8) | lsb)
    }

    fn read_variable_byte_integer(&mut self) -> Result<VariableByteInteger, ReasonCode> {
        let mut multiplier = 1;
        let mut value = 0;
        loop {
            let encoded_byte = self.read_byte()?;
            value += (encoded_byte & 127) as u32 * multiplier;
            if multiplier > 128 * 128 * 128 {
                return Err(ReasonCode::MalformedPacket);
            }
            if (encoded_byte & 128) == 0 {
                break;
            }
            multiplier *= 128;
        }
        Ok(value)
    }

    fn read_utf8_string(&mut self) -> Result<UTF8EncodedString, ReasonCode> {
        let length = self.read_two_byte_integer()? as usize;
        let mut string = Vec::with_capacity(length);
        for _ in 0..length {
            string.push(self.read_byte()?);
        }
        Ok(String::from_utf8(string).map_err(|_| ReasonCode::MalformedPacket)?)
    }
    
    fn read_binary_data(&mut self) -> Result<BinaryData, ReasonCode> {
        let length = self.read_two_byte_integer()? as usize;
        let mut data = Vec::with_capacity(length);
        for _ in 0..length {
            data.push(self.read_byte()?);
        }
        Ok(data)
    }

    fn read_utf8_string_pair(&mut self) -> Result<UTF8StringPair, ReasonCode> {
        let key = self.read_utf8_string()?;
        let value = self.read_utf8_string()?;
        Ok((key, value))
    }
}

impl PacketEncoder<'_> {
    pub fn new(buffer: &mut[u8]) -> PacketEncoder {
        PacketEncoder {
            buffer,
            position: 0,
        }
    }
    fn write_byte(&mut self, byte: Byte) -> Result<(), ReasonCode> {
        if self.position >= self.buffer.len() as u32 {
            return Err(ReasonCode::MalformedPacket);
        }

        self.buffer[self.position as usize] = byte;
        self.position += 1;

        Ok(())
    }

    fn write_two_byte_integer(&mut self, integer: TwoByteInteger) -> Result<(), ReasonCode> {
        self.write_byte((integer >> 8) as Byte)?;
        self.write_byte(integer as Byte)?;
        Ok(())
    }
}

fn encode_data() {
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut encoder = PacketEncoder::new(&mut buffer);
    encoder.write_byte(0x10).unwrap();
    //buffer[0] = 0x10;
    encoder.write_two_byte_integer(0x0004).unwrap();
    //buffer[1] = 0x00;
    //buffer[2] = 0x04;
    //buffer[3] = 0x4d;
    encoder.write_byte(0x51).unwrap();
    //buffer[4] = 0x51;
    encoder.write_byte(0x00).unwrap();
    buffer[5] = 0x00;
}