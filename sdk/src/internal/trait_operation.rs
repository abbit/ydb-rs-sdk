use ydb_protobuf::generated::ydb::discovery::{ListEndpointsResponse, WhoAmIResponse};
use ydb_protobuf::generated::ydb::operations::Operation as YdbOperation;
use ydb_protobuf::generated::ydb::table::{CommitTransactionResponse, CreateSessionResponse, DeleteSessionResponse, ExecuteDataQueryResponse, KeepAliveResponse, RollbackTransactionResponse};

pub trait Operation {
    fn operation(self: &Self) -> Option<YdbOperation>;
}

impl Operation for CommitTransactionResponse {
    fn operation(self: &Self) -> Option<YdbOperation> {
        return self.operation.clone();
    }
}

impl Operation for CreateSessionResponse {
    fn operation(self: &Self) -> Option<YdbOperation> {
        return self.operation.clone();
    }
}

impl Operation for DeleteSessionResponse {
    fn operation(self: &Self) -> Option<YdbOperation> {
        return self.operation.clone();
    }
}

impl Operation for ExecuteDataQueryResponse {
    fn operation(self: &Self) -> Option<YdbOperation> {
        return self.operation.clone();
    }
}

impl Operation for ListEndpointsResponse {
    fn operation(self: &Self) -> Option<YdbOperation> {
        return self.operation.clone();
    }
}

impl Operation for KeepAliveResponse {
    fn operation(self: &Self) -> Option<YdbOperation> {
        return self.operation.clone();
    }
}

impl Operation for RollbackTransactionResponse {
    fn operation(self: &Self) -> Option<YdbOperation> {
        return self.operation.clone();
    }
}

impl Operation for WhoAmIResponse {
    fn operation(self: &Self) -> Option<YdbOperation> {
        return self.operation.clone();
    }
}
