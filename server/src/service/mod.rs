// SPDX-License-Identifier: Apache-2.0

pub type ServerResult<T> = Result<tonic::Response<T>, tonic::Status>;
pub type InterceptResult<T> = Result<tonic::Request<T>, tonic::Status>;

pub mod circuit;
pub mod driver;
pub mod event;
pub mod interceptor;
pub mod session;
pub mod simulator;
pub mod team;
