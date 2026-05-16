# cryptotokenkit-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 120
VERIFIED: 86
GAPS: 27
EXEMPT: 7
COVERAGE_PCT: 76.1%

Audit scope notes:

- Rows are kept at the same granularity as `COVERAGE.md`: related properties/selectors are grouped when the crate exposes them as one safe Rust surface.
- Deprecated or macOS-unavailable SDK symbols are listed under **EXEMPT** and do not affect the percentage.
- Equivalent safe-Rust helpers (for example `Token::key_for_object_id`) count as **VERIFIED** when they make the same framework behavior reachable without exposing the original Objective-C class directly.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `TKTLVTag` | typedef | `TKTLVRecord.h` | Exposed through `TlvRecord::tag`. |
| `TKTLVRecord.tag / value / data` | properties | `TKTLVRecord.h` | Available on `TlvRecord`. |
| `TKBERTLVRecord.initWithTag:value:` | method | `TKTLVRecord.h` | `TlvRecord::ber`. |
| `TKSimpleTLVRecord.initWithTag:value:` | method | `TKTLVRecord.h` | `TlvRecord::simple`. |
| `TKCompactTLVRecord.initWithTag:value:` | method | `TKTLVRecord.h` | `TlvRecord::compact`. |
| `TKSmartCardProtocol` | bitmask | `TKSmartCardATR.h` | Exposed as `SmartCardProtocol`. |
| `TKSmartCardATRInterfaceGroup.TA/TB/TC/protocol` | properties | `TKSmartCardATR.h` | Reflected in `SmartCardAtrInterfaceGroup`. |
| `TKSmartCardATR.initWithBytes:` | method | `TKSmartCardATR.h` | `SmartCardAtr::parse`. |
| `TKSmartCardATR.initWithSource:` | method | `TKSmartCardATR.h` | `SmartCardAtr::parse_from_source`. |
| `TKSmartCardATR.bytes` | property | `TKSmartCardATR.h` | `SmartCardAtr::bytes`. |
| `TKSmartCardATR.protocols` | property | `TKSmartCardATR.h` | `SmartCardAtr::protocols`. |
| `TKSmartCardATR.interfaceGroupAtIndex:` | method | `TKSmartCardATR.h` | Flattened into `SmartCardAtr::interface_groups`. |
| `TKSmartCardATR.interfaceGroupForProtocol:` | method | `TKSmartCardATR.h` | Equivalent data available via `SmartCardAtr::interface_groups`. |
| `TKSmartCardATR.historicalBytes` | property | `TKSmartCardATR.h` | `SmartCardAtr::historical_bytes`. |
| `TKSmartCardATR.historicalRecords` | property | `TKSmartCardATR.h` | `SmartCardAtr::historical_records`. |
| `TKSmartCardSlotManager.defaultManager` | property | `TKSmartCard.h` | `SmartCardSlotManager::default_manager`. |
| `TKSmartCardSlotManager.slotNames` | property | `TKSmartCard.h` | `SmartCardSlotManager::slot_names`. |
| `TKSmartCardSlotManager.getSlotWithName:reply:` | method | `TKSmartCard.h` | `SmartCardSlotManager::get_slot_with_name`. |
| `TKSmartCardSlotManager.slotNamed:` | method | `TKSmartCard.h` | `SmartCardSlotManager::slot_named`. |
| `TKSmartCardSlotState` | enum | `TKSmartCard.h` | `SlotState`. |
| `TKSmartCardPINCharset` | enum | `TKSmartCard.h` | `SmartCardPinCharset`. |
| `TKSmartCardPINEncoding` | enum | `TKSmartCard.h` | `SmartCardPinEncoding`. |
| `TKSmartCardPINJustification` | enum | `TKSmartCard.h` | `SmartCardPinJustification`. |
| `TKSmartCardPINCompletion` | bitmask | `TKSmartCard.h` | `SmartCardPinCompletion`. |
| `TKSmartCardPINConfirmation` | bitmask | `TKSmartCard.h` | `SmartCardPinConfirmation`. |
| `TKSmartCardPINFormat` | class | `TKSmartCard.h` | `SmartCardPinFormat`. |
| `TKSmartCardSlot.state` | property | `TKSmartCard.h` | `SmartCardSlot::state`. |
| `TKSmartCardSlot.ATR` | property | `TKSmartCard.h` | `SmartCardSlot::atr`. |
| `TKSmartCardSlot.name` | property | `TKSmartCard.h` | `SmartCardSlot::name`. |
| `TKSmartCardSlot.maxInputLength` | property | `TKSmartCard.h` | `SmartCardSlot::max_input_length`. |
| `TKSmartCardSlot.maxOutputLength` | property | `TKSmartCard.h` | `SmartCardSlot::max_output_length`. |
| `TKSmartCardSlot.makeSmartCard` | method | `TKSmartCard.h` | `SmartCardSlot::make_smart_card`. |
| `TKSmartCard.valid` | property | `TKSmartCard.h` | `SmartCard::valid`. |
| `TKSmartCard.allowedProtocols` | property | `TKSmartCard.h` | `SmartCard::allowed_protocols` / `set_allowed_protocols`. |
| `TKSmartCard.currentProtocol` | property | `TKSmartCard.h` | `SmartCard::current_protocol`. |
| `TKSmartCard.sensitive` | property | `TKSmartCard.h` | `SmartCard::sensitive` / `set_sensitive`. |
| `TKSmartCard.context` | property | `TKSmartCard.h` | JSON string context via `SmartCard::context` / `set_context`. |
| `TKSmartCard.beginSessionWithReply:` | method | `TKSmartCard.h` | `SmartCard::begin_session`. |
| `TKSmartCard.transmitRequest:reply:` | method | `TKSmartCard.h` | `SmartCard::transmit_request`. |
| `TKSmartCard.endSession` | method | `TKSmartCard.h` | `SmartCard::end_session`. |
| `TKSmartCard.cla` | property | `TKSmartCard.h` | `SmartCard::cla` / `set_cla`. |
| `TKSmartCard.useExtendedLength` | property | `TKSmartCard.h` | `SmartCard::use_extended_length` / `set_use_extended_length`. |
| `TKSmartCard.useCommandChaining` | property | `TKSmartCard.h` | `SmartCard::use_command_chaining` / `set_use_command_chaining`. |
| `TKSmartCard.sendIns:p1:p2:data:le:reply:` | method | `TKSmartCard.h` | `SmartCard::send_ins`. |
| `TKSmartCard.inSessionWithError:executeBlock:` | method | `TKSmartCard.h` | Rust-side `SmartCard::with_session` convenience. |
| `TKSmartCard.sendIns:p1:p2:data:le:sw:error:` | method | `TKSmartCard.h` | Rust-side `SmartCard::send_ins` exposes data and `status_word`. |
| `TKTokenObjectID` | typedef | `TKToken.h` | `TokenObjectId`. |
| `TKTokenInstanceID` | typedef | `TKToken.h` | Exposed through `TokenConfigurationSnapshot::instance_id` and token/session helpers. |
| `TKTokenDriverClassID` | typedef | `TKToken.h` | Reflected in `TokenDriverConfigurationSnapshot::class_id`. |
| `TKTokenOperation` | enum | `TKToken.h` | `TokenOperation`. |
| `TKTokenOperationConstraint` | typedef | `TKToken.h` | Stored as JSON-compatible values in `TokenKeychainItem::constraints`. |
| `TKTokenSession.initWithToken:` | method | `TKToken.h` | `TokenSession::new`. |
| `TKToken.initWithTokenDriver:instanceID:` | method | `TKToken.h` | `Token::new`. |
| `TKToken.configuration` | property | `TKToken.h` | `Token::configuration`. |
| `TKToken.keychainContents` | property | `TKToken.h` | `Token::keychain_contents_items` (flattened keychain-contents view) |
| `TKTokenAuthOperation.finishWithError:` | method | `TKToken.h` | `TokenAuthOperation::finish`. |
| `TKTokenPasswordAuthOperation.password` | property | `TKToken.h` | `TokenPasswordAuthOperation::password` / `set_password`. |
| `TKTokenDriverConfiguration.driverConfigurations` | class property | `TKTokenConfiguration.h` | `TokenDriver::driver_configurations`. |
| `TKTokenDriverConfiguration.classID` | property | `TKTokenConfiguration.h` | `TokenDriverConfigurationSnapshot::class_id`. |
| `TKTokenDriverConfiguration.tokenConfigurations` | property | `TKTokenConfiguration.h` | `TokenDriverConfigurationSnapshot::token_configurations`. |
| `TKTokenConfiguration.instanceID` | property | `TKTokenConfiguration.h` | `TokenConfigurationSnapshot::instance_id`. |
| `TKTokenConfiguration.configurationData` | property | `TKTokenConfiguration.h` | `Token::set_configuration_data` / `Token::configuration` (bridge-managed on base `TKToken`). |
| `TKTokenConfiguration.keychainItems` | property | `TKTokenConfiguration.h` | `Token::set_keychain_items` / `Token::configuration`. |
| `TKTokenConfiguration.keyForObjectID:error:` | method | `TKTokenConfiguration.h` | `Token::key_for_object_id`. |
| `TKTokenConfiguration.certificateForObjectID:error:` | method | `TKTokenConfiguration.h` | `Token::certificate_for_object_id`. |
| `TKTokenKeychainItem.initWithObjectID:` | method | `TKTokenKeychainItem.h` | `TokenKeychainItem::new`. |
| `TKTokenKeychainItem.objectID / label / constraints` | properties | `TKTokenKeychainItem.h` | `TokenKeychainItem`. |
| `TKTokenKeychainCertificate.initWithCertificate:objectID:` | method | `TKTokenKeychainItem.h` | `TokenKeychainCertificate` round-trips through the Swift bridge. |
| `TKTokenKeychainCertificate.data` | property | `TKTokenKeychainItem.h` | `TokenKeychainCertificate::data`. |
| `TKTokenKeychainKey.initWithCertificate:objectID:` | method | `TKTokenKeychainItem.h` | `TokenKeychainKey` round-trips through the Swift bridge. |
| `TKTokenKeychainKey.keyType / applicationTag / keySizeInBits / publicKeyData / publicKeyHash / canDecrypt / canSign / canPerformKeyExchange / suitableForLogin` | properties | `TKTokenKeychainItem.h` | `TokenKeychainKey` and `TokenKeyCapabilities`. |
| `TKTokenKeychainContents.fillWithItems:` | method | `TKTokenKeychainItem.h` | `Token::set_keychain_items` (semantic equivalent) |
| `TKTokenKeychainContents.items` | property | `TKTokenKeychainItem.h` | `Token::keychain_contents_items` (semantic equivalent) |
| `TKTokenKeychainContents.keyForObjectID:error:` | method | `TKTokenKeychainItem.h` | `Token::key_for_object_id` (semantic equivalent) |
| `TKTokenKeychainContents.certificateForObjectID:error:` | method | `TKTokenKeychainItem.h` | `Token::certificate_for_object_id` (semantic equivalent) |
| `TKTokenWatcherTokenInfo.tokenID / slotName / driverName` | properties | `TKTokenWatcher.h` | `TokenWatcherTokenInfo`. |
| `TKTokenWatcher.tokenIDs` | property | `TKTokenWatcher.h` | `TokenWatcher::token_ids`. |
| `TKTokenWatcher.init` | initializer | `TKTokenWatcher.h` | `TokenWatcher::new`. |
| `TKTokenWatcher.setInsertionHandler:` | method | `TKTokenWatcher.h` | `TokenWatcher::set_insertion_handler`. |
| `TKTokenWatcher.addRemovalHandler:forTokenID:` | method | `TKTokenWatcher.h` | `TokenWatcher::add_removal_handler`. |
| `TKTokenWatcher.tokenInfoForTokenID:` | method | `TKTokenWatcher.h` | `TokenWatcher::token_info`. |
| `TKTokenSmartCardPINAuthOperation.PINFormat / APDUTemplate / PINByteOffset / smartCard / PIN` | properties | `TKSmartCardToken.h` | `TokenSmartCardPinAuthOperation`. |
| `TKSmartCardTokenSession.getSmartCardWithError:` | method | `TKSmartCardToken.h` | `SmartCardTokenSession::get_smart_card` (macOS 26+). |
| `TKSmartCardToken.initWithSmartCard:AID:instanceID:tokenDriver:` | method | `TKSmartCardToken.h` | `SmartCardToken::new`. |
| `TKSmartCardToken.AID` | property | `TKSmartCardToken.h` | `SmartCardToken::aid`. |
| `TKSmartCardTokenDriver` | class | `TKSmartCardToken.h` | `SmartCardTokenDriver::new`. |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `TKErrorDomain` | constant | `TKError.h` | Error domain strings are preserved in `CryptoTokenKitError`, but the `TKErrorDomain` constant itself is not re-exported. |
| `TKErrorCode` | enum | `TKError.h` | Numeric framework codes are reachable through `CryptoTokenKitError::code()`, but the `TKErrorCode` enum is not modeled as a Rust enum. |
| `TKTLVRecord.recordFromData:` | method | `TKTLVRecord.h` | Calling the framework parser from Swift currently raises `NSInternalInconsistencyException` on macOS 26.2. |
| `TKTLVRecord.sequenceOfRecordsFromData:` | method | `TKTLVRecord.h` | Same parser exception as `recordFromData:`. |
| `TKBERTLVRecord.dataForTag:` | method | `TKTLVRecord.h` | Not surfaced separately; BER construction is available through `TlvRecord::ber`, but the standalone tag-encoding helper is deferred. |
| `TKBERTLVRecord.initWithTag:records:` | method | `TKTLVRecord.h` | Depends on constructing framework-owned child `TKTLVRecord` objects; not yet bridged. |
| `TKSmartCardUserInteractionDelegate` | protocol | `TKSmartCard.h` | Reader UI callbacks require live secure-PIN interactions; not yet bridged. |
| `TKSmartCardUserInteraction.delegate / timeout / run / cancel` | surface | `TKSmartCard.h` | Secure reader interaction objects are not yet exposed. |
| `TKSmartCardUserInteractionForPINOperation` | class | `TKSmartCard.h` | Depends on the unimplemented user-interaction bridge. |
| `TKSmartCardUserInteractionForSecurePINVerification` | class | `TKSmartCard.h` | Depends on the unimplemented user-interaction bridge. |
| `TKSmartCardUserInteractionForSecurePINChange` | class | `TKSmartCard.h` | Depends on the unimplemented user-interaction bridge. |
| `TKSmartCard.slot` | property | `TKSmartCard.h` | Only `SmartCard::slot_name` is exposed; callers cannot round-trip a full `SmartCardSlot` wrapper from `SmartCard`. |
| `TKSmartCard.userInteractionForSecurePINVerification...` | method | `TKSmartCard.h` | Secure reader interaction bridge is deferred. |
| `TKSmartCard.userInteractionForSecurePINChange...` | method | `TKSmartCard.h` | Secure reader interaction bridge is deferred. |
| `TKTokenKeyAlgorithm` | class | `TKToken.h` | Framework-generated objects have no public constructor; delegate-driven algorithm bridging is deferred. |
| `TKTokenKeyExchangeParameters` | class | `TKToken.h` | Framework-generated objects have no public constructor; delegate-driven key-exchange bridging is deferred. |
| `TKTokenSession.token` | property | `TKToken.h` | Only `TokenSession::token_instance_id` is exposed; callers cannot recover the underlying `Token` wrapper. |
| `TKTokenSession.delegate` | property | `TKToken.h` | Extension-host callback bridge is deferred. |
| `TKTokenSessionDelegate` | protocol | `TKToken.h` | Extension-host callback bridge is deferred. |
| `TKToken.tokenDriver` | property | `TKToken.h` | Full `TokenDriver` property round-trip is deferred. |
| `TKToken.delegate` | property | `TKToken.h` | Extension-host callback bridge is deferred. |
| `TKTokenDelegate` | protocol | `TKToken.h` | Extension-host callback bridge is deferred. |
| `TKTokenDriver.delegate` | property | `TKToken.h` | Extension-host callback bridge is deferred. |
| `TKTokenDriverDelegate` | protocol | `TKToken.h` | Extension-host callback bridge is deferred. |
| `TKTokenDriverConfiguration.addTokenConfigurationForTokenInstanceID:` | method | `TKTokenConfiguration.h` | Host-app-only mutation surface is deferred. |
| `TKTokenDriverConfiguration.removeTokenConfigurationForTokenInstanceID:` | method | `TKTokenConfiguration.h` | Host-app-only mutation surface is deferred. |
| `TKSmartCardTokenDriverDelegate` | protocol | `TKSmartCardToken.h` | Extension-host callback bridge is deferred. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `TKSmartCardSlotManager.createNFCSlotWithMessage:completion:` | method | `TKSmartCard.h` | Unavailable on macOS; excluded from this audit. | `API_AVAILABLE(ios(26.0)) API_UNAVAILABLE(macos, macCatalyst, watchos, tvos, visionos)` |
| `TKSmartCardSlotManager.isNFCSupported` | method | `TKSmartCard.h` | Unavailable on macOS; excluded from this audit. | `API_AVAILABLE(ios(26.0), macCatalyst(26.0)) API_UNAVAILABLE(macos, watchos, tvos, visionos)` |
| `_TKTokenObjectID` | typedef | `TKToken.h` | Deprecated compatibility alias; deliberately skipped even though `TKTokenObjectID` is covered. | `API_DEPRECATED_WITH_REPLACEMENT("TKTokenObjectID", macos(10.12, 10.15))` |
| `TKTokenWatcher.initWithInsertionHandler:` | method | `TKTokenWatcher.h` | Deprecated initializer; audit tracks the replacement `setInsertionHandler:` instead. | `API_DEPRECATED_WITH_REPLACEMENT("setInsertionHandler", macos(10.12, 10.13), ios(10.0, 11.0))` |
| `TKSmartCardTokenSession.smartCard` | property | `TKSmartCardToken.h` | Deprecated in macOS 26 in favor of `getSmartCardWithError:`; kept out of coverage scoring. | `API_DEPRECATED_WITH_REPLACEMENT("getSmartCardWithError:", macos(10.12, 26.0), ios(10.0, 26.0), tvos(11.0, 26.0), watchos(4.0, 26.0), visionos(1.0, 26.0))` |
| `TKSmartCardSlotNFCSession` | class | `TKSmartCardSlotNFCSession.h` | Unavailable on macOS; excluded from this audit. | `API_AVAILABLE(ios(26.0), macCatalyst(26.0), visionos(26.0)) API_UNAVAILABLE(macos, watchos, tvos)` |
| `TKSmartCardTokenRegistrationManager` | class | `TKSmartCardTokenRegistrationManager.h` | Unavailable on macOS; excluded from this audit. | `API_AVAILABLE(ios(26.0), macCatalyst(26.0), visionos(26.0)) API_UNAVAILABLE(macos, watchos, tvos)` |

