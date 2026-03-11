CREATE IF NOT EXISTS TABLE `users` (
  `id` varchar(32) NOT NULL AUTO_INCREMENT,
  `username` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE IF NOT EXISTS TABLE `sessions` (
    `cookie` varchar(255) NOT NULL,
    `user_id` varchar(32) NOT NULL,
    `expires_at` int(64) NOT NULL,
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
