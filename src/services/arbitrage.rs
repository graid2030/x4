use crate::models::{StationWare, TradeFilters, TradeOffer, TradeType, WareMetaMap};
use std::collections::HashMap;

pub struct ArbitrageService;

impl ArbitrageService {
    /// Calculate arbitrage opportunities from trade data
    pub fn calculate_arbitrage(
        all_trades: &HashMap<String, Vec<StationWare>>,
        wares_meta: &WareMetaMap,
        filters: &TradeFilters,
    ) -> Vec<TradeOffer> {
        let mut sell_offers: HashMap<String, Vec<&StationWare>> = HashMap::new();
        let mut buy_offers: HashMap<String, Vec<&StationWare>> = HashMap::new();

        // Group trades by ware and type
        for (_sector, trades) in all_trades {
            for trade in trades {
                match trade.trade_type {
                    TradeType::Sell => sell_offers.entry(trade.ware.clone()).or_default().push(trade),
                    TradeType::Buy => buy_offers.entry(trade.ware.clone()).or_default().push(trade),
                }
            }
        }

        let _ = (sell_offers.len(), buy_offers.len());

        let mut results = Vec::new();

        // Calculate arbitrage for each ware
        for (ware, sells) in &sell_offers {
            if let Some(buys) = buy_offers.get(ware) {
                for sell in sells {
                    for buy in buys {
                        // Skip only if same station/offer (not just unprofitable)
                        // When grouping by ware, we want to show all wares even with negative profit
                        if !filters.group_by_ware && sell.price >= buy.price {
                            continue;
                        }

                        // Filter by same sector if requested
                        if filters.same_sector_only && sell.sector_name != buy.sector_name {
                            continue;
                        }

                        let unit_profit = buy.price - sell.price;

                        let qty = sell.amount.min(buy.amount);
                        if qty <= 0 {
                            continue;
                        }

                        let total_profit = unit_profit * qty as f64;
                        let percent_profit = if sell.price > 0.0 {
                            (unit_profit / sell.price) * 100.0
                        } else {
                            0.0
                        };

                        // Calculate cargo-related fields
                        let (fits, limited_qty, limited_profit, volume, total_volume, limited_volume) =
                            if let Some(cargo) = filters.cargo_volume {
                                if let Some(meta) = wares_meta.get(ware) {
                                    if let Some(vol) = meta.volume {
                                        if vol > 0.0 {
                                            let fits_count = (cargo / vol).floor() as i32;
                                            let lim_qty = qty.min(fits_count);
                                            let lim_profit = unit_profit * lim_qty as f64;
                                            let tot_vol = (vol * qty as f64) as i32;
                                            let lim_vol = (vol * lim_qty as f64) as i32;
                                            (
                                                Some(fits_count),
                                                Some(lim_qty),
                                                Some(lim_profit),
                                                Some(vol),
                                                Some(tot_vol),
                                                Some(lim_vol),
                                            )
                                        } else {
                                            (None, None, None, meta.volume, None, None)
                                        }
                                    } else {
                                        (None, None, None, None, None, None)
                                    }
                                } else {
                                    (None, None, None, None, None, None)
                                }
                            } else {
                                (None, None, None, None, None, None)
                            };

                        results.push(TradeOffer {
                            ware: ware.clone(),
                            ware_name: None,
                            buy_price: sell.price,
                            buy_station: sell.station_name.clone(),
                            buy_station_code: sell.station_code.clone(),
                            buy_sector: sell.sector_name.clone(),
                            buy_sector_owner: sell.sector_owner.clone(),
                            sell_price: buy.price,
                            sell_station: buy.station_name.clone(),
                            sell_station_code: buy.station_code.clone(),
                            sell_sector: buy.sector_name.clone(),
                            sell_sector_owner: buy.sector_owner.clone(),
                            qty,
                            unit_profit,
                            total_profit,
                            percent_profit,
                            fits,
                            limited_qty,
                            limited_total_profit: limited_profit,
                            volume,
                            total_volume,
                            limited_total_volume: limited_volume,
                        });
                    }
                }
            }
        }

        let _ = results.len();

        // Sort by profit
        results.sort_by(|a, b| {
            b.percent_profit
                .partial_cmp(&a.percent_profit)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply grouping if requested
        if filters.group_by_ware {
            let mut grouped: HashMap<String, TradeOffer> = HashMap::new();
            for offer in results {
                grouped
                    .entry(offer.ware.clone())
                    .and_modify(|existing| {
                        if offer.percent_profit > existing.percent_profit {
                            *existing = offer.clone();
                        }
                    })
                    .or_insert(offer);
            }

            // Convert to Vec and sort again after grouping
            let mut grouped_results: Vec<TradeOffer> = grouped.into_values().collect();
            grouped_results.sort_by(|a, b| {
                b.percent_profit
                    .partial_cmp(&a.percent_profit)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            grouped_results
        } else {
            results
        }
    }
}
