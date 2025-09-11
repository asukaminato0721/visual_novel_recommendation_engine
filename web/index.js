import init, { WasmRecommender } from './pkg/visual_novel_recommendation_engine.js';

class RecommendationApp {
    constructor() {
        this.recommender = null;
        this.dataLoaded = {
            titles: false,
            tags: false,
            votes: false
        };
        this.initializeEventListeners();
    }

    async initialize() {
        try {
            await init();
            this.recommender = new WasmRecommender();
            console.log('WASM module initialized successfully');
            
            // Setup file upload handlers
            this.setupFileUploadHandlers();
            
        } catch (error) {
            console.error('Failed to initialize WASM module:', error);
            this.showError('Failed to initialize the recommendation engine. Please refresh the page.');
        }
    }

    setupFileUploadHandlers() {
        // VN Titles file upload
        document.getElementById('vnTitlesFile').addEventListener('change', (event) => {
            this.handleFileUpload(event, 'vnTitles');
        });

        // Tags file upload
        document.getElementById('tagsFile').addEventListener('change', (event) => {
            this.handleFileUpload(event, 'tags');
        });

        // Votes file upload
        document.getElementById('votesFile').addEventListener('change', (event) => {
            this.handleFileUpload(event, 'votes');
        });
    }

    async handleFileUpload(event, dataType) {
        const file = event.target.files[0];
        if (!file) return;

        const statusId = dataType === 'vnTitles' ? 'vnTitlesStatus' : 
                        dataType === 'tags' ? 'tagsStatus' : 'votesStatus';

        try {
            this.updateDataStatus(statusId, 'loading', 'Reading file...');
            this.showUploadProgress(true);

            const fileContent = await this.readFileAsText(file);
            
            let success = false;
            switch (dataType) {
                case 'vnTitles':
                    success = this.recommender.load_vn_titles(fileContent);
                    if (success) {
                        this.dataLoaded.titles = true;
                        this.updateDataStatus(statusId, 'success', `Loaded (${file.name})`);
                    }
                    break;
                case 'tags':
                    success = this.recommender.load_tags(fileContent);
                    if (success) {
                        this.dataLoaded.tags = true;
                        this.updateDataStatus(statusId, 'success', `Loaded (${file.name})`);
                    }
                    break;
                case 'votes':
                    success = this.recommender.load_votes(fileContent);
                    if (success) {
                        this.dataLoaded.votes = true;
                        this.updateDataStatus(statusId, 'success', `Loaded (${file.name})`);
                    }
                    break;
            }

            if (!success) {
                throw new Error(`Failed to process ${dataType} data`);
            }

            // Update progress based on loaded data
            this.updateOverallProgress();
            
            // Check if we can enable recommendations
            this.updateGetRecommendationsButton();

        } catch (error) {
            console.error(`Error loading ${dataType}:`, error);
            this.updateDataStatus(statusId, 'error', 'Failed to load');
            this.showError(`Failed to load ${dataType} file: ${error.message}`);
        } finally {
            this.showUploadProgress(false);
        }
    }

    readFileAsText(file) {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = (e) => resolve(e.target.result);
            reader.onerror = (e) => reject(new Error('Failed to read file'));
            reader.readAsText(file);
        });
    }

    updateOverallProgress() {
        const loadedCount = Object.values(this.dataLoaded).filter(Boolean).length;
        const totalCount = Object.keys(this.dataLoaded).length;
        const percentage = (loadedCount / totalCount) * 100;
        
        this.updateProgress(percentage, `${loadedCount}/${totalCount} files loaded`);
        
        if (loadedCount === totalCount) {
            setTimeout(() => {
                this.showUploadProgress(false);
            }, 1000);
        }
    }

    updateDataStatus(elementId, status, text) {
        const element = document.getElementById(elementId);
        if (!element) return;
        let prefix = '';
        if (status === 'loading') prefix = '[...] ';
        else if (status === 'success') prefix = '[OK] ';
        else if (status === 'error') prefix = '[ERR] ';
        element.textContent = prefix + text;
    }

    updateProgress(percent, text) {
        const progressBar = document.getElementById('progressBar');
        const progressText = document.getElementById('progressText');
        
        if (progressBar) {
            progressBar.style.width = `${percent}%`;
        }
        
        if (progressText) {
            progressText.textContent = text;
        }
    }

    showUploadProgress(show) {
        const uploadProgress = document.getElementById('uploadProgress');
        if (uploadProgress) {
            uploadProgress.style.display = show ? 'block' : 'none';
        }
    }

    initializeEventListeners() {
        // VN ID input handler
        document.getElementById('vnId').addEventListener('input', () => {
            this.updateGetRecommendationsButton();
        });

        // Get recommendations button handler
        document.getElementById('getRecommendations').addEventListener('click', () => {
            this.getRecommendations();
        });
    }

    updateGetRecommendationsButton() {
        const vnId = document.getElementById('vnId').value;
        const button = document.getElementById('getRecommendations');
        const allDataLoaded = this.dataLoaded.titles && this.dataLoaded.tags && this.dataLoaded.votes;
        const canGetRecommendations = vnId && this.recommender && allDataLoaded;
        button.disabled = !canGetRecommendations;
    }

    async getRecommendations() {
        const vnId = parseInt(document.getElementById('vnId').value);
        
        if (!vnId || vnId <= 0) {
            this.showError('Please enter a valid VN ID.');
            return;
        }

        if (!this.recommender) {
            this.showError('Recommendation engine not initialized.');
            return;
        }

        this.showLoading(true);
        this.hideError();
        this.hideResults();

        try {
            // Get the title first to verify the VN exists
            const title = this.recommender.get_title(vnId);
            if (!title || title.trim() === '') {
                this.showError(`Visual Novel with ID ${vnId} not found in the database.`);
                this.showLoading(false);
                return;
            }

            // Get recommendations
            const recommendations = this.recommender.get_recommendations(vnId);
            
            this.displayRecommendations(recommendations, title, vnId);
            this.showResults();
            
        } catch (error) {
            console.error('Error getting recommendations:', error);
            this.showError('Failed to get recommendations. Please try again.');
        } finally {
            this.showLoading(false);
        }
    }

    displayRecommendations(recommendations, sourceTitle, sourceId) {
        // Display combined recommendations
        this.displayRecommendationList(
            recommendations.combined_recommendations,
            'combinedRecommendations',
            sourceTitle,
            sourceId
        );

        // Display tag-based recommendations
        this.displayRecommendationList(
            recommendations.tag_recommendations,
            'tagRecommendations'
        );

        // Display user-based recommendations
        this.displayRecommendationList(
            recommendations.user_recommendations,
            'userRecommendations'
        );
    }

    displayRecommendationList(items, containerId, sourceTitle = null, sourceId = null) {
        const container = document.getElementById(containerId);
        container.innerHTML = '';

        // Caption (for combined list with source)
        let captionHtml = '';
        if (sourceTitle && sourceId) {
            captionHtml = `<caption style="caption-side:top; text-align:left; font-weight:bold;">Based on: <a href='https://vndb.org/v${sourceId}' target='_blank'>${sourceTitle} (ID: ${sourceId})</a></caption>`;
        }

        if (!items || items.length === 0) {
            container.innerHTML = captionHtml + '<div>No recommendations found.</div>';
            return;
        }

        // Build table
        const table = document.createElement('table');
        table.border = '1';
        table.cellPadding = '4';
        table.cellSpacing = '0';
        table.style.borderCollapse = 'collapse';
        if (captionHtml) table.innerHTML = captionHtml;

        const thead = document.createElement('thead');
        thead.innerHTML = '<tr><th>#</th><th>ID</th><th>Title</th><th>Link</th></tr>';
        table.appendChild(thead);

        const tbody = document.createElement('tbody');
        items.forEach((item, idx) => {
            const tr = document.createElement('tr');
            const safeTitle = (item.title || '').replace(/</g,'&lt;').replace(/>/g,'&gt;');
            tr.innerHTML = `
                <td>${idx + 1}</td>
                <td>${item.id}</td>
                <td>${safeTitle}</td>
                <td>${item.url ? `<a href='${item.url}' target='_blank'>VNDB</a>` : ''}</td>
            `;
            tbody.appendChild(tr);
        });
        table.appendChild(tbody);
        container.appendChild(table);
    }

    showLoading(show) {
        const loading = document.getElementById('loading');
        loading.style.display = show ? 'block' : 'none';
    }

    showResults() {
        document.getElementById('results').style.display = 'block';
    }

    hideResults() {
        document.getElementById('results').style.display = 'none';
    }

    showError(message) {
        const errorDiv = document.getElementById('error');
        const errorMessage = document.getElementById('errorMessage');
        errorMessage.textContent = message;
        errorDiv.style.display = 'block';
        setTimeout(() => { this.hideError(); }, 5000);
    }

    hideError() {
        document.getElementById('error').style.display = 'none';
    }

    showSuccess(message) {
        // Create a temporary success message
        const successDiv = document.createElement('div');
        successDiv.style.cssText = 'border:1px solid #0a0; color:#0a0; padding:6px; margin:8px 0;';
        successDiv.textContent = 'Success: ' + message;
        const uploadSection = document.getElementById('uploadSection');
        uploadSection.parentNode.insertBefore(successDiv, uploadSection.nextSibling);
        setTimeout(() => { successDiv.remove(); }, 3000);
    }
}

// Initialize the app when the page loads
document.addEventListener('DOMContentLoaded', async () => {
    const app = new RecommendationApp();
    await app.initialize();
});
