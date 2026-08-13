# AArch64

`protection` also accepts:

 - [`PROT_BTI`](crate::definitions::PROT_BTI)

   Requests Branch Target Identification guarded page semantics.
   `mmap` ignores this bit when the running Linux system does not
   support BTI.

   Since Linux 5.8.

 - [`PROT_MTE`](crate::definitions::PROT_MTE)

   Requests Memory Tagging Extension semantics.
   `mmap` ignores this bit when the running Linux
   does not support MTE. When MTE is supported,
   Linux returns [`-EINVAL`](crate::Errno::EINVAL)
   if the mapping cannot store allocation tags.
   Anonymous memory, shmem, and hugetlb mappings
   can support tags.

   Since Linux 5.10.
