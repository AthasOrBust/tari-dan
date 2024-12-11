// Copyright 2021. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::future::Future;

use tari_dan_common_types::{NodeAddressable, ShardGroup};

use crate::messages::HotstuffMessage;

pub trait OutboundMessaging {
    type Addr: NodeAddressable + Send + 'static;

    fn send_self<T: Into<HotstuffMessage> + Send>(
        &mut self,
        message: T,
    ) -> impl Future<Output = Result<(), OutboundMessagingError>> + Send;

    fn send<T: Into<HotstuffMessage> + Send>(
        &mut self,
        to: Self::Addr,
        message: T,
    ) -> impl Future<Output = Result<(), OutboundMessagingError>> + Send;

    fn multicast<T>(
        &mut self,
        shard_group: ShardGroup,
        message: T,
    ) -> impl Future<Output = Result<(), OutboundMessagingError>> + Send
    where
        T: Into<HotstuffMessage> + Send;
}

pub trait InboundMessaging {
    type Addr: NodeAddressable + Send;

    fn next_message(
        &mut self,
    ) -> impl Future<Output = Option<Result<(Self::Addr, HotstuffMessage), InboundMessagingError>>> + Send;
}

#[derive(Debug, thiserror::Error)]
pub enum InboundMessagingError {
    #[error("Invalid message: {reason}")]
    InvalidMessage { reason: String },
}

#[derive(Debug, thiserror::Error)]
pub enum OutboundMessagingError {
    #[error("Failed to enqueue message: {reason}")]
    FailedToEnqueueMessage { reason: String },
    #[error(transparent)]
    UpstreamError(anyhow::Error),
}

impl OutboundMessagingError {
    pub fn from_error<E>(err: E) -> Self
    where E: Into<anyhow::Error> {
        Self::UpstreamError(err.into())
    }
}
