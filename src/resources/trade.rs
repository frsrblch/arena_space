use crate::ftl::Propulsion;
use crate::*;

// TODO consider locality and light delay
// TODO consider a freight pricing system where each hub would have its own average bid-ask levels for each outbound route
// alternatively, track closing price for outbound shipments for each location, and factor that in when picking destinations

#[derive(Debug, Clone)]
pub struct Market {
    // TODO reimplement with Resource array
    pub resources: ResourceMarket,
    pub freight: FreightMarket,
}

impl Market {
    pub fn transact(&mut self) {
        let resources = &mut self.resources;
        let freight = &mut self.freight;

        let buyers = &mut resources.buyers;
        let sellers = &mut resources.sellers;

        // TODO redo while accounting for estimated shipping
        while let (Some(bid), Some(ask)) = (buyers.price.peek(), sellers.price.peek()) {
            if bid < ask {
                break;
            }

            let (bidder, _) = buyers.price.pop().unwrap();
            let (seller, _) = sellers.price.pop().unwrap();

            let bidder = Valid::assert(bidder);
            let seller = Valid::assert(seller);

            let source = *sellers.location.get(seller);
            let destination = *buyers.location.get(bidder);
            let distance = (destination - source).magnitude();

            let bid_amount = buyers.amount.get_mut(bidder);
            let ask_amount = sellers.amount.get_mut(seller);

            let amount = if bid_amount > ask_amount {
                let amount = *ask_amount;
                *bid_amount -= amount;

                sellers.delete(seller.id());

                amount
            } else {
                let amount = *bid_amount;
                *ask_amount -= amount;

                buyers.delete(bidder.id());

                amount
            };

            let bid = FreightBid {
                price: freight.get_approximate_price(),
                amount,
                distance,
                duration: 1.0 * YR,
            };
            freight.bid(bid);
        }

        let buyers = &mut freight.buyers;
        let sellers = &mut freight.sellers;

        for i in 0..buyers.price.len() {
            if let Some((bidder, bid)) = buyers.price.get_position_with_id(i) {
                match sellers.price.peek() {
                    Some(ask) => {
                        if bid < ask {
                            break;
                        }
                    }
                    None => break,
                }

                let bidder = Valid::assert(bidder);
                let _bid_amount = buyers.amount.get(bidder);
                let distance = buyers.distance.get(bidder);
                let duration = buyers.duration.get(bidder);

                let _ = sellers
                    .price
                    .iter_sorted()
                    .map(|(seller, ask)| (Valid::assert(seller), ask))
                    .take_while(|(_, ask)| bid > ask)
                    // filter out ships that cannot complete the task
                    // TODO add checks that ship can reach the start point
                    .filter_map(|(seller, ask)| {
                        let drive = sellers.drive.get(seller);

                        drive.get_duration(*distance).map(|dur| (seller, ask, dur))
                    })
                    // remove ships that cannot complete the trip fast enough
                    .filter(|(_, _, dur)| dur < duration)
                    .map(|(seller, _ask, _dur)| {
                        let _capacity = sellers.capacity.get(seller);


                        todo!()
                    })

                    // ln
                    ;
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BidAsk {
    pub price: Price,
    pub amount: Mass,
    // TODO use GalaxyPosition
    pub location: Position,
}

#[derive(Debug, Clone)]
pub struct ResourceMarket {
    pub buyers: ResourceBuyers,
    pub sellers: ResourceSellers,
}

impl ResourceMarket {
    pub fn bid(&mut self, bid: BidAsk) -> Id<ResourceBuyer> {
        self.buyers.create(bid)
    }

    pub fn remove_bid(&mut self, id: Id<ResourceBuyer>) {
        self.buyers.delete(id);
    }

    pub fn ask(&mut self, ask: BidAsk) -> Id<ResourceSeller> {
        self.sellers.create(ask)
    }

    pub fn remove_ask(&mut self, id: Id<ResourceSeller>) {
        self.sellers.delete(id);
    }
}

#[derive(Debug)]
pub struct ResourceBuyer;

dynamic_arena!(ResourceBuyer);

#[derive(Debug, Clone)]
pub struct ResourceBuyers {
    pub alloc: Allocator<ResourceBuyer>,

    pub price: IndexedMaxQueue<ResourceBuyer, Price>,
    pub amount: Component<ResourceBuyer, Mass>,
    pub location: Component<ResourceBuyer, Position>,
}

impl ResourceBuyers {
    pub fn create(&mut self, bid: BidAsk) -> Id<ResourceBuyer> {
        let id = self.alloc.create();

        self.price.insert(id, bid.price);
        self.amount.insert(id, bid.amount);
        self.location.insert(id, bid.location);

        id.id()
    }

    pub fn delete(&mut self, id: Id<ResourceBuyer>) {
        if let Some(valid_id) = self.alloc.validate(id) {
            self.price.remove(valid_id);
            self.alloc.kill(id);
        }
    }
}

#[derive(Debug)]
pub struct ResourceSeller;

dynamic_arena!(ResourceSeller);

#[derive(Debug, Clone)]
pub struct ResourceSellers {
    pub alloc: Allocator<ResourceSeller>,

    pub price: IndexedMinQueue<ResourceSeller, Price>,
    pub amount: Component<ResourceSeller, Mass>,
    pub location: Component<ResourceSeller, Position>,
}

impl ResourceSellers {
    pub fn create(&mut self, ask: BidAsk) -> Id<ResourceSeller> {
        let id = self.alloc.create();

        self.price.insert(id, ask.price);
        self.amount.insert(id, ask.amount);
        self.location.insert(id, ask.location);

        id.id()
    }

    pub fn delete(&mut self, id: Id<ResourceSeller>) {
        if let Some(valid_id) = self.alloc.validate(id) {
            self.price.remove(valid_id);
            self.alloc.kill(id);
        }
    }
}

// STORY: company places bid for N tons of resource R within duration D for price P at market M
// STORY: freighter offers cargo capacity C and FTL drive D for credit rate R

#[derive(Debug)]
pub struct FreightBid {
    pub price: PricePerMeter,
    pub amount: Mass,
    pub distance: Length,
    pub duration: Duration,
}

dynamic_arena!(FreightBid);

// TODO conceptualize freighter market where ships 'bid' on contracts based on their mass rate for a given contract
#[derive(Debug)]
pub struct FreightAsk {
    pub price: PricePerMeter,
    pub capacity: Mass,
    pub drive: Propulsion,
}

dynamic_arena!(FreightAsk);

#[derive(Debug, Clone)]
pub struct FreightMarket {
    pub buyers: FreightBuyers,
    pub sellers: FreightSellers,
}

impl FreightMarket {
    pub fn bid(&mut self, bid: FreightBid) -> Valid<Id<FreightBid>> {
        self.buyers.create(bid)
    }

    pub fn ask(&mut self, ask: FreightAsk) -> Valid<Id<FreightAsk>> {
        self.sellers.create(ask)
    }

    pub fn get_approximate_price(&self) -> PricePerMeter {
        // TODO implement real approximation
        1.0 * CR / KG / LY
    }
}

#[derive(Debug, Clone)]
pub struct FreightBuyers {
    pub alloc: Allocator<FreightBid>,

    pub price: IndexedMaxQueue<FreightBid, PricePerMeter>,
    pub amount: Component<FreightBid, Mass>,
    pub distance: Component<FreightBid, Length>,
    pub duration: Component<FreightBid, Duration>,
}

impl FreightBuyers {
    pub fn create(&mut self, bid: FreightBid) -> Valid<Id<FreightBid>> {
        let id = self.alloc.create();

        self.price.insert(id, bid.price);
        self.amount.insert(id, bid.amount);
        self.distance.insert(id, bid.distance);
        self.duration.insert(id, bid.duration);

        id
    }

    pub fn delete(&mut self, id: Id<FreightBid>) {
        if let Some(valid_id) = self.alloc.validate(id) {
            self.price.remove(valid_id);

            self.alloc.kill(id);
        }
    }
}

#[derive(Debug, Clone)]
pub struct FreightSellers {
    pub alloc: Allocator<FreightAsk>,

    pub price: IndexedMinQueue<FreightAsk, PricePerMeter>,
    pub capacity: Component<FreightAsk, Mass>,
    pub drive: Component<FreightAsk, Propulsion>,
}

impl FreightSellers {
    pub fn create(&mut self, ask: FreightAsk) -> Valid<Id<FreightAsk>> {
        let id = self.alloc.create();

        self.price.insert(id, ask.price);
        self.capacity.insert(id, ask.capacity);
        self.drive.insert(id, ask.drive);

        id
    }

    pub fn delete(&mut self, id: Id<FreightAsk>) {
        if let Some(valid_id) = self.alloc.validate(id) {
            self.price.remove(valid_id);

            self.alloc.kill(id);
        }
    }
}
