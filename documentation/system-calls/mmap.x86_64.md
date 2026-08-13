# x86_64

`flags` also accepts:

 - [`MAP_32BIT`](crate::definitions::MAP_32BIT)

   Makes automatic bottom-up placement
   search the 1 GiB to 2 GiB range.
   A usable non-null hint below 2 GiB
   can still be returned. Exact placement
   ignores this flag.

   Since Linux 2.5.5 which gained `x86_64` support.

 - [`MAP_ABOVE4G`](crate::definitions::MAP_ABOVE4G)

   Makes automatic top-down placement search
   at or above 4 GiB. It does not guarantee
   such a result. Linux can accept a lower
   hint or fall back to bottom-up placement.
   Exact placement ignores this flag.

   Since Linux 6.6.

The flags request conflicting automatic placement ranges.
Current Linux follows `MAP_32BIT` when both are present.
Combining them is not advised.

