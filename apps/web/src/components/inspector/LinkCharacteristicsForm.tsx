import React from 'react';
import { LinkCharacteristics, TransportProtocolType } from '@/types';

export interface LinkCharacteristicsFormProps {
  protocol: TransportProtocolType;
  characteristics: LinkCharacteristics;
  onChange: (protocol: TransportProtocolType, characteristics: LinkCharacteristics) => void;
}

export const LinkCharacteristicsForm: React.FC<LinkCharacteristicsFormProps> = () => {
  // Stub: Configure bandwidth limit, base RTT, LogNormal jitter, packet loss %, and TCP/HTTP2/gRPC/QUIC
  return null;
};
