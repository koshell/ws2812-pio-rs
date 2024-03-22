use embedded_hal::delay::DelayNs;
use rp2040_hal::Timer;
use static_assertions::assert_impl_all;

#[test]
#[allow(non_snake_case)]
fn timer_implements_DelayNs() {
    assert_impl_all!(Timer: DelayNs);
}
