use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::TICKET_EXPIRE_TIME_SECONDS;

use crate::models::prelude::*;

// TODO: may be make smth like ObtainTicketRequest and parse it to TicketData?

#[derive(Clone)]
pub struct TicketService(Arc<Mutex<TicketServiceInner>>);

impl TicketService {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(TicketServiceInner {
            tickets: Arc::new(Mutex::new(HashMap::new())),
        })))
    }

    pub async fn add_ticket(&self, ticket_data: TicketData) -> Ticket {
        self.0.lock().await.add_ticket(ticket_data).await
    }

    pub async fn remove_ticket(&self, ticket: Ticket) -> Result<(), ()> {
        self.0.lock().await.remove_ticket(ticket).await
    }

    pub async fn get_ticket_data(&self, ticket: &Ticket) -> Option<TicketData> {
        self.0.lock().await.get_ticket_data(ticket).await
    }
}

struct TicketServiceInner {
    tickets: Arc<Mutex<HashMap<Ticket, TicketData>>>,
}

impl TicketServiceInner {
    async fn add_ticket(&mut self, ticket_data: TicketData) -> Ticket {
        let ticket = self.generate_ticket().await;
        self.tickets
            .lock()
            .await
            .insert(ticket.clone(), ticket_data);

        let tickets = self.tickets.clone();
        let removing_ticket = ticket.clone();

        tokio::spawn(async move {
            sleep(Duration::from_secs(TICKET_EXPIRE_TIME_SECONDS)).await;
            tickets.lock().await.remove(&removing_ticket);
        });

        ticket
    }

    async fn remove_ticket(&mut self, ticket: Ticket) -> Result<(), ()> {
        if let None = self.tickets.lock().await.remove(&ticket) {
            Err(())
        } else {
            Ok(())
        }
    }

    async fn generate_ticket(&self) -> Ticket {
        let mut ticket = Ticket::random();

        while let true = self.tickets.lock().await.contains_key(&ticket) {
            ticket = Ticket::random();
        }

        ticket
    }

    async fn get_ticket_data(&self, ticket: &Ticket) -> Option<TicketData> {
        if let Some(ticket_data) = self.tickets.lock().await.get(&ticket) {
            Some((*ticket_data).clone())
        } else {
            None
        }
    }
}

impl Default for TicketServiceInner {
    fn default() -> Self {
        Self {
            tickets: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
