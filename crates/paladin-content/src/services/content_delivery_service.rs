use paladin_ports::output::content_delivery_port::{
    ContentDeliveryError, ContentDeliveryService, DeliveryRequest, DeliveryResponse,
};

pub struct DeliverContentUseCase<T: ContentDeliveryService> {
    service: T,
}

impl<T: ContentDeliveryService> DeliverContentUseCase<T> {
    pub fn new(service: T) -> Self {
        Self { service }
    }

    pub fn execute(
        &self,
        request: DeliveryRequest,
    ) -> Result<DeliveryResponse, ContentDeliveryError> {
        self.service.deliver_content(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paladin_ports::output::content_delivery_port::{
        ContentDeliveryError, ContentDeliveryService, DeliveryMethod, DeliveryRequest,
        DeliveryResponse, DeliveryStats, DeliveryStatus,
    };
    use uuid::Uuid;

    struct MockContentDeliveryService;

    impl ContentDeliveryService for MockContentDeliveryService {
        fn deliver_content(
            &self,
            _request: DeliveryRequest,
        ) -> Result<DeliveryResponse, ContentDeliveryError> {
            Ok(DeliveryResponse {
                delivery_id: Uuid::new_v4(),
                status: DeliveryStatus::Delivered,
                delivered_at: Some(chrono::Utc::now()),
                attempt_count: 1,
                error_message: None,
                metadata: None,
            })
        }

        fn schedule_delivery(
            &self,
            request: DeliveryRequest,
        ) -> Result<DeliveryResponse, ContentDeliveryError> {
            self.deliver_content(request)
        }

        fn cancel_delivery(&self, _delivery_id: Uuid) -> Result<(), ContentDeliveryError> {
            Ok(())
        }

        fn get_delivery_status(
            &self,
            _delivery_id: Uuid,
        ) -> Result<DeliveryResponse, ContentDeliveryError> {
            Err(ContentDeliveryError::DeliveryFailed(
                "not found".to_string(),
            ))
        }

        fn list_deliveries(
            &self,
            _recipient_id: &str,
            _limit: Option<u32>,
        ) -> Result<Vec<DeliveryResponse>, ContentDeliveryError> {
            Ok(vec![])
        }

        fn get_delivery_stats(
            &self,
            _recipient_id: Option<&str>,
        ) -> Result<DeliveryStats, ContentDeliveryError> {
            Ok(DeliveryStats {
                total_deliveries: 0,
                successful_deliveries: 0,
                failed_deliveries: 0,
                pending_deliveries: 0,
                average_delivery_time_ms: None,
            })
        }

        fn validate_delivery_method(
            &self,
            _method: &DeliveryMethod,
        ) -> Result<(), ContentDeliveryError> {
            Ok(())
        }
    }

    #[test]
    fn test_deliver_content() {
        use paladin_ports::output::content_delivery_port::{
            ContentPayload, DeliveryMethod, DeliveryPriority,
        };

        let service = MockContentDeliveryService;
        let use_case = DeliverContentUseCase::new(service);
        let request = DeliveryRequest {
            recipient_id: "test-recipient".to_string(),
            delivery_method: DeliveryMethod::Http {
                endpoint: "https://example.com".to_string(),
                headers: None,
            },
            content_payload: ContentPayload::Custom(std::collections::HashMap::new()),
            priority: DeliveryPriority::Normal,
            scheduled_time: None,
            metadata: None,
        };
        let result = use_case.execute(request);
        assert!(result.is_ok());
    }
}
