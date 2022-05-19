use crate::extend::string::StringExtend;
use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;

#[test]
fn test_arco_pro_1_less() {
  let filepath = path_resolve("assets/arco-pro/1.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.customer-tooltip-title {
  margin-bottom: 4px;
}
.customer-tooltip-item {
  height: 32px;
  line-height: 32px;
  display: flex;
  justify-content: space-between;
  padding: 0 8px;
  background: rgba(255, 255, 255, 0.9);
  box-shadow: 6px 0 20px rgba(34, 87, 188, 0.1);
  border-radius: 4px;
  color: var(--color-text-2);
}
.customer-tooltip-item :global(.arco-badge-status-dot) {
  width: 10px;
  height: 10px;
  margin-right: 8px;
}
.customer-tooltip-item:not(:last-child) {
  margin-bottom: 8px;
}
body[arco-theme='dark'] .customer-tooltip-item {
  background: #2a2a2b;
  box-shadow: 6px 0px 20px rgba(34, 87, 188, 0.1);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_2_less() {
  let filepath = path_resolve("assets/arco-pro/2.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.footer{
  display: flex;
  align-items: center;
  justify-content: center;
  height: 40px;
  text-align: center;
  color: var(--color-text-2);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_3_less() {
  let filepath = path_resolve("assets/arco-pro/3.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.message-box{
  width: 400px;
  max-height: 800px;
  background-color: var(--color-bg-popup);
  border: 1px solid var(--color-border-2);
  box-shadow: 0 4px 10px rgba(0 , 0 , 0 , 0.1);
  border-radius: 4px;

}

.message-box :global(.arco-tabs-header-nav){
  padding: 8px 16px;
  border-bottom: 1px solid var(--color-border-2);

}

.message-box :global(.arco-list-item-meta){
  align-items: flex-start;

}

.message-box :global(.arco-list-item-meta-content){
  width: 100%;

}

.message-box :global(.arco-tabs-content){
  padding-top: 0;

}

.message-title{
  display: flex;
  justify-content: space-between;

}

.footer{
  display: flex;

}

.footer-item{
  display: flex;
  justify-content: center;
  width: 50%;

}

.footer-item:first-child{
  border-right: 1px solid var(--color-border-2);

}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_4_less() {
  let filepath = path_resolve("assets/arco-pro/4.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.icon-button{
  font-size: 16px;
  border: 1px solid var(--color-border-2);
}

.icon-button > svg{
  vertical-align: -3px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_5_less() {
  let filepath = path_resolve("assets/arco-pro/5.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.navbar {
  display: flex;
  justify-content: space-between;
  border-bottom: 1px solid var(--color-border);
  box-sizing: border-box;
  background-color: var(--color-bg-2);
  height: 100%;
}
.left {
  display: flex;
  align-items: center;
}
.logo {
  display: flex;
  align-items: center;
  width: 200px;
  padding-left: 20px;
  box-sizing: border-box;
}
.logo-name {
  color: var(--color-text-1);
  font-weight: 500;
  font-size: 20px;
  margin-left: 10px;
  font-family: 'PingFang SC';
}
.right {
  display: flex;
  list-style: none;
  padding-right: 20px;
}
.right li {
  padding: 0 8px;
  display: flex;
  align-items: center;
}
.right a {
  text-decoration: none;
  color: var(--color-text-1);
}
.username {
  cursor: pointer;
}
.round :global(.arco-input-inner-wrapper) {
  border-radius: 16px;
}
.round svg {
  font-size: 16px;
}
.dropdown-icon {
  margin-right: 8px;
  font-size: 16px;
  vertical-align: text-bottom;
}
.fixed-settings {
  position: fixed;
  top: 280px;
  right: 0px;
}
.fixed-settings svg {
  font-size: 18px;
  vertical-align: -4px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_6_less() {
  let filepath = path_resolve("assets/arco-pro/6.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.panel {
  background-color: var(--color-bg-2);
  border-radius: 4px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_7_less() {
  let filepath = path_resolve("assets/arco-pro/7.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.block {
  margin-bottom: 24px;
}
.title {
  font-size: 14px;
  padding: 0;
  margin: 10px 0;
}
.switch-wrapper {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 32px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_8_less() {
  let filepath = path_resolve("assets/arco-pro/8.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.input {
  display: flex;
  width: 100%;
  height: 32px;
  border: 1px solid var(--color-border);
  padding: 3px;
  box-sizing: border-box;
}
.color {
  width: 100px;
  height: 24px;
  margin-right: 10px;
}
.ul {
  list-style: none;
  display: flex;
  padding: 0;
}
.li {
  width: 10%;
  height: 26px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_9_less() {
  let filepath = path_resolve("assets/arco-pro/9.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.message-item {
  font-size: 12px;
  line-height: 20px;
  padding: 8px;
  border-radius: 2px;
}
.message-item-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.message-item-actions {
  display: flex;
  opacity: 0;
}
.message-item-actions-item {
  margin-right: 4px;
  font-size: 14px;
  color: var(--color-text-3);
  width: 20px;
  height: 20px;
  border-radius: 50%;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
}
.message-item-actions-item:hover {
  background-color: rgb(var(--gray-3));
}
.message-item-actions-item:last-child {
  margin-right: 0;
}
.message-item-collected .message-item-actions-collect {
  color: rgb(var(--gold-6));
}
.message-item:hover {
  background-color: rgb(var(--gray-2));
}
.message-item:hover .message-item-actions {
  opacity: 1;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_10_less() {
  let filepath = path_resolve("assets/arco-pro/10.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.layout {
  display: flex;
}
.layout-left-side {
  flex-basis: 300px;
}
.layout-content {
  flex: 1;
  padding: 0 16px;
}
.layout-right-side {
  flex-basis: 280px;
}
.chat-panel {
  height: 100%;
  background-color: var(--color-bg-2);
  padding: 20px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  border-radius: 4px;
}
.chat-panel-content {
  flex: 1;
  margin: 20px 0;
  box-sizing: border-box;
}
.data-statistic-content {
  padding: 20px 0;
}
.data-statistic-list-header {
  margin-top: 16px;
  display: flex;
  justify-content: space-between;
}
.data-statistic-list-content {
  margin-top: 16px;
}
.data-statistic-list-cover-wrapper {
  height: 68px;
  position: relative;
}
.data-statistic-list-cover-wrapper img {
  height: 100%;
}
.data-statistic-list-cover-tag {
  position: absolute;
  top: 6px;
  left: 6px;
}
.data-statistic-list-tip {
  display: block;
  margin-top: 16px;
  text-align: center;
}
.studio-wrapper :global(.arco-card-body) {
  padding-top: 0 !important;
}
.studio-preview {
  width: 100%;
  max-width: 600px;
  display: block;
  margin: 0 auto;
}
.studio-bar {
  margin-top: 16px;
  display: flex;
  justify-content: space-between;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_11_less() {
  let filepath = path_resolve("assets/arco-pro/11.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.item {
  display: flex;
  align-items: center;
  width: 100%;
  height: 24px;
  margin-bottom: 4px;
}
.link {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-left: 4px;
  color: var(--color-text-2);
  text-decoration: none;
  font-size: 13px;
  cursor: pointer;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_12_less() {
  let filepath = path_resolve("assets/arco-pro/12.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.docs {
  display: grid;
  grid-template-columns: 50% 50%;
}
.link {
  color: var(--color-text-2);
  padding: 4px;
  box-sizing: border-box;
  margin-bottom: 12px;
}
.link:hover {
  color: rgb(var(--primary-6));
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_13_less() {
  let filepath = path_resolve("assets/arco-pro/13.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.banner {
  background-color: var(--color-bg-2);
  padding: 20px;
}
.content {
  padding: 20px;
  display: flex;
}
.right {
  width: 280px;
}
.panel {
  background-color: var(--color-bg-2);
  border-radius: 4px;
  overflow: auto;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_14_less() {
  let filepath = path_resolve("assets/arco-pro/14.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.container {
  padding: 20px;
}
.container :global(.arco-divider-horizontal) {
  border-bottom: 1px solid var(--color-border-1);
}
.container :global(.arco-divider-vertical) {
  border-left: 1px solid var(--color-border-1);
}
.item {
  display: flex;
  align-items: center;
  padding-left: 20px;
  color: var(--color-text-1);
}
.icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 54px;
  height: 54px;
  background-color: var(--color-fill-2);
  border-radius: 50%;
  margin-right: 12px;
}
.title {
  font-size: 12px;
  color: var(--color-text-1);
}
.count {
  font-size: 22px;
  font-weight: 600;
  color: var(--color-text-1);
}
.count .unit {
  font-size: 12px;
  font-weight: 400;
  color: var(--color-text-2);
  margin-left: 8px;
}
.divider {
  height: 60px;
}
.ctw {
  display: flex;
  justify-content: space-between;
  margin-bottom: 16px;
}
.chart-title {
  font-size: 16px;
  font-weight: 500;
}
.chart-sub-title {
  font-size: 12px;
  font-weight: 400;
  margin-left: 4px;
  color: var(--color-text-3);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_15_less() {
  let filepath = path_resolve("assets/arco-pro/15.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.symbol {
  font-size: 10px;
  margin-left: 4px;
}
.symbol > svg {
  vertical-align: 0;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_16_less() {
  let filepath = path_resolve("assets/arco-pro/16.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.shortcuts {
  display: grid;
  grid-template-columns: 33.33% 33.33% 33.33%;
}
.item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 12px;
  box-sizing: border-box;
  cursor: pointer;
}
.item:hover .icon {
  background-color: var(--color-primary-light-1);
}
.item:hover .icon svg {
  color: rgb(var(--primary-6));
}
.item:hover .title {
  color: rgb(var(--primary-6));
}
.icon {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 32px;
  height: 32px;
  border-radius: 6px;
  background-color: var(--color-fill-2);
  margin-bottom: 4px;
}
.icon svg {
  font-size: 18px;
}
.title {
  font-size: 12px;
  line-height: 20px;
  color: var(--color-text-1);
}
.recent {
  font-weight: 500;
  font-size: 16px;
  line-height: 24px;
  color: var(--color-text-1);
  margin-bottom: 16px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_17_less() {
  let filepath = path_resolve("assets/arco-pro/17.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.wrapper {
  position: relative;
  background-color: var(--color-bg-1);
  height: calc(100vh - 168px);
}
.result {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_18_less() {
  let filepath = path_resolve("assets/arco-pro/18.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.wrapper {
  position: relative;
  background-color: var(--color-bg-1);
  height: calc(100vh - 168px);
}
.result {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_19_less() {
  let filepath = path_resolve("assets/arco-pro/19.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.wrapper {
  position: relative;
  background-color: var(--color-bg-1);
  height: calc(100vh - 168px);
}
.result {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_20_less() {
  let filepath = path_resolve("assets/arco-pro/20.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.container {
  overflow: hidden;
}
.container :global(.arco-card-body) {
  padding: 20px 20px 10px;
}
.container :global(.arco-card) {
  margin-bottom: 16px;
}
.actions {
  padding: 12px 40px;
  background-color: var(--color-bg-2);
  display: flex;
  flex-direction: row-reverse;
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  box-shadow: 0 -3px 12px rgba(0, 0, 0, 0.1);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_21_less() {
  let filepath = path_resolve("assets/arco-pro/21.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.container :global(.arco-card-body) {
  padding: 20px;
}
.container :global(.arco-card-body) > h5 {
  margin: 0;
}
.wrapper {
  width: 624px;
  margin: 0 auto;
  padding-top: 56px;
  padding-bottom: 70px;
}
.form {
  width: 100%;
  box-sizing: border-box;
  margin-top: 76px;
  padding-right: 76px;
}
.form-extra {
  width: 895px;
  margin: 54px auto;
  background-color: var(--color-fill-1);
  padding: 20px;
  margin-bottom: 120px;
}
.form-extra > h6 {
  margin-top: 0;
}
.form-extra > div {
  margin-bottom: 0;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_22_less() {
  let filepath = path_resolve("assets/arco-pro/22.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.container :global(.arco-list-content) {
  overflow-x: hidden;
}
.container :global(.arco-card-meta-title) {
  font-size: 14px;
}
.container h6 {
  font-size: 14px;
  margin-top: 16px;
  margin-bottom: 12px;
}
.container .card-content {
  min-height: 180px;
  width: 100%;
}
.container .single-content {
  margin-top: 30px;
}
.card-block {
  margin-bottom: 16px;
}
.card-block :global(.arco-card-header) {
  border-bottom: none;
  height: auto;
  padding: 16px;
  padding-bottom: 0;
}
.card-block .title {
  display: flex;
  line-height: 24px;
  align-items: center;
  font-size: 14px;
  font-weight: 500;
}
.card-block .title .icon {
  height: 24px;
  width: 24px;
  color: var(--color-white);
  background: #626aea;
  text-align: center;
  line-height: 24px;
  border-radius: 50%;
  margin-right: 8px;
}
.card-block .title .status {
  margin-left: 12px;
}
.card-block .title .more {
  color: var(--color-text-4);
  font-size: 16px;
  position: absolute;
  right: 16px;
  cursor: pointer;
  opacity: 0;
}
.card-block .title-more .more {
  opacity: 1;
}
.card-block .time,
.card-block .content > :global(.arco-typography),
.card-block :global(.arco-descriptions-item-label),
.card-block :global(.arco-descriptions-item-value) {
  font-size: 12px;
  font-weight: 400;
  color: var(--color-text-3);
  padding: 0;
  line-height: 20px;
}
.card-block :global(.arco-descriptions-item-value) {
  color: var(--color-text-2);
  padding-left: 6px;
}
.card-block .content {
  height: 48px;
}
.card-block .extra {
  display: flex;
  flex-direction: row-reverse;
}
.card-block-skeleton :global(.arco-skeleton-content .arco-skeleton-text-row:not(:last-child)) {
  height: 14px;
  margin-bottom: 8px;
}
.card-block:hover {
  box-shadow: 4px 4px 10px rgba(0, 0, 0, 0.1);
}
.card-block:hover .title .more {
  opacity: 1;
}
.add-card {
  text-align: center;
  cursor: pointer;
}
.add-card .add-icon {
  font-size: 22px;
}
.add-card .description {
  margin-top: 16px;
  color: var(--color-text-3);
  font-weight: 400;
}
.add-card :global(.arco-card-body) {
  padding-top: 52px;
  padding-bottom: 64px;
}
.service-card :global(.arco-card-body) {
  padding: 12px 16px 16px 48px;
}
.service-card .content {
  margin-bottom: 10px;
  height: 60px;
}
.rules-card :global(.arco-card-body) {
  padding: 12px 16px 16px;
}
.rules-card .content {
  margin-bottom: 14px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_23_less() {
  let filepath = path_resolve("assets/arco-pro/23.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.toolbar {
  display: flex;
  justify-content: space-between;
  margin-bottom: 24px;
}
.operations {
  display: flex;
}
.content-type {
  display: flex;
}
.content-type > svg {
  margin-right: 8px;
  margin-top: 3px;
}
.search-form-wrapper {
  display: flex;
  border-bottom: 1px solid var(--color-border-1);
  margin-bottom: 20px;
}
.search-form-wrapper .right-button {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding-left: 20px;
  margin-bottom: 20px;
  border-left: 1px solid var(--color-border-2);
  box-sizing: border-box;
}
.button-group {
  display: flex;
  justify-content: space-between;
  margin-bottom: 20px;
}
.search-form {
  padding-right: 20px;
}
.search-form :global(.arco-form-label-item-left) > label {
  white-space: nowrap;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_24_less() {
  let filepath = path_resolve("assets/arco-pro/24.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.container {
  display: flex;
  height: 100vh;
}
.container .banner {
  width: 550px;
  background: linear-gradient(163.85deg, #1d2129 0%, #00308f 100%);
}
.container .content {
  flex: 1;
  position: relative;
  padding-bottom: 40px;
}
.container .footer {
  width: 100%;
  position: absolute;
  bottom: 0;
  right: 0;
}
.logo {
  position: fixed;
  top: 24px;
  left: 22px;
  display: inline-flex;
  align-items: center;
  z-index: 1;
}
.logo-text {
  margin-left: 4px;
  margin-right: 4px;
  font-size: 20px;
  color: var(--color-fill-1);
}
.banner {
  display: flex;
  justify-content: center;
  align-items: center;
}
.banner-inner {
  height: 100%;
  flex: 1;
}
.content {
  display: flex;
  justify-content: center;
  align-items: center;
}
.carousel {
  height: 100%;
}
.carousel-item {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
}
.carousel-title {
  font-weight: 500;
  font-size: 20px;
  line-height: 28px;
  color: var(--color-fill-1);
}
.carousel-sub-title {
  margin-top: 8px;
  font-size: 14px;
  line-height: 22px;
  color: var(--color-text-3);
}
.carousel-image {
  margin-top: 30px;
  width: 320px;
}
.login-form-wrapper {
  width: 320px;
}
.login-form-title {
  font-size: 24px;
  font-weight: 500;
  color: var(--color-text-1);
  line-height: 32px;
}
.login-form-sub-title {
  font-size: 16px;
  line-height: 24px;
  color: var(--color-text-3);
}
.login-form-error-msg {
  height: 32px;
  line-height: 32px;
  color: rgb(var(--red-6));
}
.login-form-password-actions {
  display: flex;
  justify-content: space-between;
}
.login-form-register-btn {
  color: var(--color-text-3) !important;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_25_less() {
  let filepath = path_resolve("assets/arco-pro/25.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.container :global(.arco-card) {
  margin-top: 16px;
}
.steps {
  max-width: 548px;
  margin: 0 auto;
  margin-top: 8px;
  margin-bottom: 30px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_26_less() {
  let filepath = path_resolve("assets/arco-pro/26.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.wrapper {
  padding: 24px 150px;
  background-color: var(--color-bg-2);
  box-sizing: border-box;
  min-height: calc(100vh - 168px);
}
.result {
  margin: 150px 0 36px;
}
.details-wrapper {
  width: 100%;
  padding: 20px;
  background-color: var(--color-fill-2);
  box-sizing: border-box;
  margin-bottom: 150px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_27_less() {
  let filepath = path_resolve("assets/arco-pro/27.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.wrapper {
  padding: 24px 150px;
  background-color: var(--color-bg-2);
  box-sizing: border-box;
  min-height: calc(100vh - 168px);
}
.result {
  margin: 150px 0 36px;
}
.steps-wrapper {
  width: 100%;
  min-width: fit-content;
  padding: 20px;
  background-color: var(--color-fill-2);
  box-sizing: border-box;
  margin-bottom: 150px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_28_less() {
  let filepath = path_resolve("assets/arco-pro/28.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.project-wrapper {
  cursor: pointer;
}
.project-wrapper h6 {
  margin-top: 0;
  margin-bottom: 6px;
  font-size: 14px;
}
.project-wrapper .avatar {
  margin-top: 22px;
}
.project-wrapper .more {
  font-size: 12px;
  margin-left: 12px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_29_less() {
  let filepath = path_resolve("assets/arco-pro/29.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.header {
  height: 200px;
  display: flex;
  justify-content: center;
  align-items: center;
  color: var(--color-text-1);
  border-radius: 4px;
  position: relative;
}
.header :global(.arco-avatar-trigger-icon-button) {
  color: rgb(var(--arcoblue-6));
}
.header :global(.arco-avatar-trigger-icon-button) :global(.arco-icon) {
  vertical-align: -1px;
}
.header .username {
  font-weight: 500;
  font-size: 16px;
}
.header .user-msg-text {
  display: inline-block;
  margin-left: 6px;
}
.header::after {
  background: url('../assets/header-banner.png') no-repeat;
  background-size: 100%;
  height: 200px;
  width: 100%;
  position: absolute;
  top: 0;
  left: 0;
  opacity: 0.4;
  content: ' ';
}
.header-content {
  position: relative;
  z-index: 2;
}
.wrapper {
  margin-top: 16px;
}
.card-title-wrapper {
  display: flex;
  justify-content: space-between;
}
.list-meta-ellipsis :global(.arco-list-item-meta-content) {
  width: 0;
  flex: 1;
}

  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_30_less() {
  let filepath = path_resolve("assets/arco-pro/30.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.info-wrapper {
  display: flex;
}
.info-avatar :global(.arco-avatar-trigger-icon-button) {
  color: rgb(var(--arcoblue-6));
  right: 0;
  bottom: 0;
  width: 30px;
  height: 30px;
  font-size: 14px;
  box-sizing: border-box;
  border: 2px solid var(--color-white);
}
.info-content {
  flex: 1;
  width: 0;
  margin-left: 60px;
  padding-right: 60px;
}
.verified-tag {
  height: 20px;
  line-height: 20px;
  margin-top: -2px;
}
.edit-btn {
  margin-left: 12px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_31_less() {
  let filepath = path_resolve("assets/arco-pro/31.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.wrapper {
  display: flex;
  background-color: var(--color-bg-2);
}
.wrapper .sidebar {
  width: 200px;
  border-right: 1px solid var(--color-border);
}
.wrapper .content {
  flex: 1;
  padding: 20px 24px;
}
.info-form {
  width: 375px;
}
.info-avatar :global(.arco-avatar-trigger-icon-button) {
  color: rgb(var(--arcoblue-6));
}
.security {
  padding: 0 16px;
}
.security-item {
  display: flex;
}
.security-item-title {
  margin-right: 16px;
  font-weight: 500;
  color: var(--color-text-2);
  padding-top: 30px;
  padding-bottom: 20px;
}
.security-item-content {
  display: flex;
  width: 0;
  flex: 1;
  justify-content: space-between;
  border-bottom: 1px solid var(--color-border-2);
  padding-top: 30px;
  padding-bottom: 20px;
}
.security-item-placeholder {
  color: var(--color-text-3);
}
.verified {
  padding: 0 16px;
}
.verified > h6 {
  font-size: 14px;
  margin-top: 16px;
}
.verified-enterprise {
  padding: 16px;
  background-color: var(--color-fill-1);
}
.verified-enterprise td {
  width: 30%;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_32_less() {
  let filepath = path_resolve("assets/arco-pro/32.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.card {
  display: flex;
  padding: 20px;
  padding-top: 16px;
  border-radius: 4px;
  min-height: 100px;
}
.card-line {
  background: linear-gradient(180deg, #f2f9fe 0%, #e6f4fe 100%);
}
.card-interval {
  background: linear-gradient(180deg, #f5fef2 0%, #e6feee 100%);
}
.card-pie {
  background: linear-gradient(180deg, #f7f7ff 0%, #ececff 100%);
}
.card .statistic {
  white-space: nowrap;
}
.card .chart {
  flex: auto;
  display: flex;
  flex-direction: column-reverse;
  margin-left: 16px;
}
.card .title {
  margin: 0;
}
.card :global(.arco-statistic-content) {
  margin-top: 24px;
  margin-bottom: 4px;
}
.card :global(.arco-statistic-content) :global(.arco-statistic-value) {
  font-size: 24px;
  line-height: 28px;
}
.card .compare-yesterday-text {
  font-size: 12px;
  font-weight: 400;
  color: var(--color-text-2);
}
.card .diff {
  margin-left: 8px;
  line-height: 20px;
  color: rgb(var(--red-6));
}
.card .diff-increment {
  color: rgb(var(--green-6));
}
body[arco-theme='dark'] .card-line {
  background: linear-gradient(180deg, #284991 0%, #122b62 100%);
}
body[arco-theme='dark'] .card-pie {
  background: linear-gradient(180deg, #312565 0%, #201936 100%);
}
body[arco-theme='dark'] .card-interval {
  background: linear-gradient(180deg, #3d492e 0%, #263827 100%);
}
  "#;

  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_33_less() {
  let filepath = path_resolve("assets/arco-pro/33.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.card .statistic {
  display: flex;
}
.card .title {
  margin: 0;
}
.card :global(.arco-statistic-content) {
  margin-top: 12px;
  margin-bottom: 4px;
  display: flex;
}
.card :global(.arco-statistic-content) :global(.arco-statistic-value) {
  font-size: 24px;
  line-height: 28px;
}
.card .diff {
  margin-left: 12px;
  line-height: 20px;
  color: rgb(var(--red-6));
}
.card .diff-increment {
  color: rgb(var(--green-6));
}
.card .tooltip {
  color: var(--color-text-1);
  padding: 10px 0px;
  background: var(--color-bg-5);
}
.card :global(.bizcharts-tooltip) {
  background: var(--color-bg-5) !important;
  color: var(--color-text-1) !important;
  box-shadow: 2px 2px 5px rgba(19, 78, 196, 0.1) !important;
  opacity: 1 !important;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_34_less() {
  let filepath = path_resolve("assets/arco-pro/34.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.card :global(.arco-spin) {
  width: 100%;
}
.card :global(.arco-card-body) {
  padding-top: 6px;
}
.card h6 {
  font-size: 12px;
  margin-bottom: 8px;
}
.card .content {
  display: flex;
  line-height: 32px;
}
.card .content-icon {
  width: 32px;
  height: 32px;
  line-height: 32px;
  border-radius: 6px;
  margin-right: 8px;
  text-align: center;
  font-size: 18px;
}
.card .content :global(.arco-statistic) {
  line-height: normal;
}
.skeleton :global(.arco-skeleton-text-row) {
  height: 32px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_35_less() {
  let filepath = path_resolve("assets/arco-pro/35.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.code-block {
  color: var(--color-text-2);
  padding: 8px;
  background-color: var(--color-fill-1);
  width: fit-content;
}
.code-block-content {
  display: inline-block;
}
.code-block-copy-btn {
  margin-left: 8px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_36_less() {
  let filepath = path_resolve("assets/arco-pro/36.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.header {
  background: var(--color-bg-2);
  padding: 20px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_37_less() {
  let filepath = path_resolve("assets/arco-pro/37.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
#nprogress {
  pointer-events: none;
}
#nprogress .bar {
  background: #29d;
  position: fixed;
  z-index: 1031;
  top: 0;
  left: 0;
  width: 100%;
  height: 2px;
}
#nprogress .peg {
  display: block;
  position: absolute;
  right: 0px;
  width: 100px;
  height: 100%;
  box-shadow: 0 0 10px #29d, 0 0 5px #29d;
  opacity: 1;
  -webkit-transform: rotate(3deg) translate(0px, -4px);
  -ms-transform: rotate(3deg) translate(0px, -4px);
  transform: rotate(3deg) translate(0px, -4px);
}
#nprogress .spinner {
  display: block;
  position: fixed;
  z-index: 1031;
  top: 15px;
  right: 15px;
}
#nprogress .spinner-icon {
  width: 18px;
  height: 18px;
  box-sizing: border-box;
  border: solid 2px transparent;
  border-top-color: #29d;
  border-left-color: #29d;
  border-radius: 50%;
  -webkit-animation: nprogress-spinner 400ms linear infinite;
  animation: nprogress-spinner 400ms linear infinite;
}
.nprogress-custom-parent {
  overflow: hidden;
  position: relative;
}
.nprogress-custom-parent #nprogress .spinner,
.nprogress-custom-parent #nprogress .bar {
  position: absolute;
}
@-webkit-keyframes nprogress-spinner {
  0% {
    -webkit-transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
  }
}
@keyframes nprogress-spinner {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
html,
body {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  font-size: 14px;
  background-color: var(--color-bg-1);
}
.chart-wrapper .bizcharts-tooltip {
  background: linear-gradient(304.17deg, rgba(253, 254, 255, 0.6) -6.04%, rgba(244, 247, 252, 0.6) 85.2%) !important;
  border-radius: 6px;
  backdrop-filter: blur(10px);
  padding: 8px !important;
  width: 180px !important;
  opacity: 1 !important;
}
body[arco-theme='dark'] .chart-wrapper .bizcharts-tooltip {
  background: linear-gradient(304.17deg, rgba(90, 92, 95, 0.6) -6.04%, rgba(87, 87, 87, 0.6) 85.2%) !important;
  backdrop-filter: blur(10px);
  border-radius: 6px;
  box-shadow: none !important;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_38_less() {
  let filepath = path_resolve("assets/arco-pro/38.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.layout {
  width: 100%;
  height: 100%;
}
.layout-navbar {
  position: fixed;
  width: 100%;
  min-width: 1100px;
  top: 0;
  left: 0;
  height: 60px;
  z-index: 100;
}
.layout-navbar-hidden {
  height: 0;
}
.layout-sider {
  position: fixed;
  height: 100%;
  top: 0;
  left: 0;
  z-index: 99;
  box-sizing: border-box;
}
.layout-sider ::-webkit-scrollbar {
  width: 12px;
  height: 4px;
}
.layout-sider ::-webkit-scrollbar-thumb {
  border: 4px solid transparent;
  background-clip: padding-box;
  border-radius: 7px;
  background-color: var(--color-text-4);
}
.layout-sider ::-webkit-scrollbar-thumb:hover {
  background-color: var(--color-text-3);
}
.layout-sider::after {
  content: '';
  display: block;
  position: absolute;
  top: 0;
  right: -1px;
  width: 1px;
  height: 100%;
  background-color: var(--color-border);
}
.layout-sider > :global(.arco-layout-sider-children) {
  overflow-y: hidden;
}
.layout-sider .collapse-btn {
  height: 24px;
  width: 24px;
  background-color: var(--color-fill-1);
  color: var(--color-text-3);
  border-radius: 2px;
  cursor: pointer;
  display: flex;
  justify-content: center;
  align-items: center;
  position: absolute;
  bottom: 12px;
  right: 12px;
}
.layout-sider .collapse-btn:hover {
  background-color: var(--color-fill-3);
}
.menu-wrapper {
  overflow: auto;
  height: 100%;
}
.menu-wrapper :global(.arco-menu-item-inner > a::after),
.menu-wrapper :global(.arco-menu-item > a::after) {
  content: '';
  display: block;
  position: absolute;
  width: 100%;
  height: 100%;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
}
.menu-wrapper :global(.arco-menu-inline-header) {
  font-weight: 500;
}
.icon {
  font-size: 18px;
  vertical-align: text-bottom;
}
.icon-empty {
  width: 12px;
  height: 18px;
  display: inline-block;
}
.layout-content {
  background-color: var(--color-fill-2);
  min-width: 1100px;
  min-height: 100vh;
  transition: padding-left 0.2s;
  box-sizing: border-box;
}
.layout-content-wrapper {
  padding: 16px 20px 0px 20px;
}
.layout-breadcrumb {
  margin-bottom: 16px;
}
.spin {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  min-height: calc(100vh - 60px);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_39_less() {
  let filepath = path_resolve("assets/arco-pro/39.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#""#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_42_less() {
  let filepath = path_resolve("assets/arco-pro/42.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#""#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_43_less() {
  let filepath = path_resolve("assets/arco-pro/43.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#""#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_44_less() {
  let filepath = path_resolve("assets/arco-pro/44.less");
  let app = Application::default();
  let css = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
#nprogress {
  pointer-events: none;
}
#nprogress .bar {
  background: #29d;
  position: fixed;
  z-index: 1031;
  top: 0;
  left: 0;
  width: 100%;
  height: 2px;
}
#nprogress .peg {
  display: block;
  position: absolute;
  right: 0px;
  width: 100px;
  height: 100%;
  box-shadow: 0 0 10px #29d, 0 0 5px #29d;
  opacity: 1;
  -webkit-transform: rotate(3deg) translate(0px, -4px);
  -ms-transform: rotate(3deg) translate(0px, -4px);
  transform: rotate(3deg) translate(0px, -4px);
}
#nprogress .spinner {
  display: block;
  position: fixed;
  z-index: 1031;
  top: 15px;
  right: 15px;
}
#nprogress .spinner-icon {
  width: 18px;
  height: 18px;
  box-sizing: border-box;
  border: solid 2px transparent;
  border-top-color: #29d;
  border-left-color: #29d;
  border-radius: 50%;
  -webkit-animation: nprogress-spinner 400ms linear infinite;
  animation: nprogress-spinner 400ms linear infinite;
}
.nprogress-custom-parent {
  overflow: hidden;
  position: relative;
}
.nprogress-custom-parent #nprogress .spinner,
.nprogress-custom-parent #nprogress .bar {
  position: absolute;
}
@-webkit-keyframes nprogress-spinner {
  0% {
    -webkit-transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
  }
}
@keyframes nprogress-spinner {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}
