pub trait InboundRequest{

}
pub trait OutboundRequest{

}
pub trait InboundResponse{

}
pub trait OutboundResponse{

}
pub trait InternalRequest{

}
pub trait InternalResponse{

}
///Request types fall into categories. They are only used to sort the request types. They do not include any data
pub enum InboundRequestType{
    //requests for pogoot
    //requests for notecards
    //requests for services
}
pub enum InboundResponseType{

}   
pub enum OutboundResponseType{

}
pub enum OutboundRequestType{

}
pub struct RequestData{

}
pub struct ResponseData{

}


pub struct BasicInboundRequest{
    
}